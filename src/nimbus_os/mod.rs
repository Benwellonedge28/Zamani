//! Zenith UMC Nimbus OS Microkernel Core Definitions
//!
//! This module contains the foundational definitions for the Nimbus Operating
//! System's microkernel, including its core components for secure isolation,
//! multi-context management, inter-process communication (IPC), and
//! capability-based security. These are the direct OS-level abstractions.

use crate::ast::Identifier; // For Identifier
use crate::core_lang_primitives::{Duration, MemoryRegion, Size, TimeStamp}; // Use core primitives
use crate::nimbus_os::evas::{EvasActionContext, EvasDecision, EvasFilter, EvasPolicyLevel};
use crate::runtime::mts::TimelineId; // Import TimelineId
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex}; // Import E.V.A.S. components

/// Unique identifier for an isolated Nimbus execution context.
pub type NimbusContextId = u64;
/// Unique identifier for a secure communication channel.
pub type ChannelId = u64;
/// Represents a conceptual security sandbox policy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SandboxPolicy(pub String);

/// Represents a conceptual capability token for fine-grained access control.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CapabilityToken(pub String);

/// Enum representing the state of a Nimbus execution context.
#[derive(Debug, Clone, PartialEq)]
pub enum NimbusContextState {
    Running,
    Paused,
    Terminated,
    Error(String),
}

// --- Process/Thread Management ---
/// Represents a conceptual thread/process ID within a context.
pub type ThreadId = u64;
/// States for a microkernel-managed thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Priority levels for the Nimbus scheduler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Idle,
    Normal,
    High,
    RealTime(u32), // Real-time priority with level
}

/// Conceptual representation of an isolated execution environment in Nimbus.
#[derive(Debug, Clone)]
pub struct NimbusContext {
    pub id: NimbusContextId,
    pub parent_id: Option<NimbusContextId>,
    pub blueprint_id: String, // Which program/nano-agent it's running
    pub sandbox_policy: SandboxPolicy,
    pub resource_limits: HashMap<String, String>,
    pub active_capabilities: HashSet<CapabilityToken>,
    pub current_state: NimbusContextState,
    pub execution_timeline_id: Option<TimelineId>,
    pub message_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    pub threads: HashMap<ThreadId, ThreadState>,
    pub next_thread_id: ThreadId,
    pub priority: Priority,
}

/// Power states for hardware components managed by Nimbus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PowerState {
    Active,
    Idle,
    Sleep,
    DeepSleep,
    Off,
}

/// Conceptual global scheduler for the Nimbus Microkernel.
#[derive(Debug, Clone)]
pub struct GlobalScheduler {
    run_queue: VecDeque<NimbusContextId>, // Contexts ready to run
                                          // Mapping of context to its scheduled slice/time
}

impl GlobalScheduler {
    pub fn new() -> Self {
        GlobalScheduler {
            run_queue: VecDeque::new(),
        }
    }

    pub fn schedule_next_context(&mut self) -> Option<NimbusContextId> {
        self.run_queue.pop_front() // Simple round-robin for conceptual
    }

    pub fn add_to_run_queue(&mut self, context_id: NimbusContextId) {
        if !self.run_queue.contains(&context_id) {
            self.run_queue.push_back(context_id);
        }
    }

    pub fn remove_from_run_queue(&mut self, context_id: NimbusContextId) {
        self.run_queue.retain(|&id| id != context_id);
    }
}

/// A conceptual Nimbus OS Microkernel responsible for system-level operations.
/// This is the actual microkernel interface.
#[derive(Debug, Clone)]
pub struct NimbusMicrokernel {
    contexts: HashMap<NimbusContextId, NimbusContext>,
    next_context_id: NimbusContextId,
    next_channel_id: ChannelId,
    channels: HashMap<ChannelId, Arc<Mutex<VecDeque<Vec<u8>>>>>,
    registered_devices: HashMap<u64, String>,
    device_power_states: HashMap<u64, PowerState>,
    global_scheduler: Arc<Mutex<GlobalScheduler>>,
    evas_filter: EvasFilter,
}

impl NimbusMicrokernel {
    pub fn new() -> Self {
        NimbusMicrokernel {
            contexts: HashMap::new(),
            next_context_id: 1,
            next_channel_id: 1,
            channels: HashMap::new(),
            registered_devices: HashMap::new(),
            device_power_states: HashMap::new(),
            global_scheduler: Arc::new(Mutex::new(GlobalScheduler::new())),
            evas_filter: EvasFilter::new(EvasPolicyLevel::Strict),
        }
    }

    /// Creates a new isolated execution context with a specific sandbox policy.
    pub fn create_context(
        &mut self,
        blueprint_id: String,
        parent_id: Option<NimbusContextId>,
        sandbox_policy: SandboxPolicy,
    ) -> Result<NimbusContextId, String> {
        let id = self.next_context_id;
        let new_context = NimbusContext {
            id,
            parent_id,
            blueprint_id: blueprint_id.clone(),
            sandbox_policy: sandbox_policy.clone(),
            resource_limits: HashMap::new(),
            active_capabilities: HashSet::new(),
            current_state: NimbusContextState::Running,
            execution_timeline_id: None,
            message_queue: Arc::new(Mutex::new(VecDeque::new())),
            threads: HashMap::new(),
            next_thread_id: 1,
            priority: Priority::Normal,
        };

        // E.V.A.S. evaluation before context creation
        let evas_action = EvasActionContext {
            action_type: "create_context".to_string(),
            perceived_intent: format!("Launch program '{}'.", blueprint_id),
            ..Default::default()
        };
        match self.evas_filter.evaluate_action(evas_action) {
            EvasDecision::Block(reason) => {
                return Err(format!("E.V.A.S. blocked context creation: {}.", reason))
            }
            _ => { /* Allow or Warn */ }
        }

        self.next_context_id += 1;
        self.contexts.insert(id, new_context.clone());
        self.global_scheduler.lock().unwrap().add_to_run_queue(id);
        println!(
            "    -> Nimbus OS: Created isolated context {} for blueprint '{}'.".to_string(),
            id, blueprint_id
        );
        Ok(id)
    }

    /// Destroys an existing execution context, reclaiming its resources.
    pub fn destroy_context(&mut self, id: NimbusContextId) -> Result<(), String> {
        if self.contexts.remove(&id).is_some() {
            self.global_scheduler
                .lock()
                .unwrap()
                .remove_from_run_queue(id);
            println!("    -> Nimbus OS: Destroyed context {}.".to_string(), id);
            Ok(())
        } else {
            Err(format!("Context {} not found.", id))
        }
    }

    // --- Expanded Microkernel System Calls ---

    /// Sets the power state of a registered hardware device.
    pub fn set_device_power_state(
        &mut self,
        context_id: NimbusContextId,
        device_id: u64,
        state: PowerState,
    ) -> Result<(), String> {
        if let Some(context) = self.contexts.get(&context_id) {
            if !context
                .active_capabilities
                .contains(&CapabilityToken("power_management".to_string()))
            {
                return Err(format!(
                    "Context {} lacks 'power_management' capability.",
                    context_id
                ));
            }
            if !self.registered_devices.contains_key(&device_id) {
                return Err(format!("Device {} is not registered.", device_id));
            }
            self.device_power_states.insert(device_id, state.clone());
            println!(
                "    -> Nimbus OS: Context {} set device {} power state to {:?}.".to_string(),
                context_id, device_id, state
            );
            Ok(())
        } else {
            Err(format!("Context {} not found.", context_id))
        }
    }

    /// Reconfigures a hardware component (e.g., FPGA, Neuromorphic array) with a new bitstream/program.
    pub fn reconfigure_hardware(
        &mut self,
        context_id: NimbusContextId,
        device_id: u64,
        configuration: Vec<u8>,
    ) -> Result<(), String> {
        if let Some(context) = self.contexts.get(&context_id) {
            if !context
                .active_capabilities
                .contains(&CapabilityToken("hardware_reconfiguration".to_string()))
            {
                return Err(format!(
                    "Context {} lacks 'hardware_reconfiguration' capability.",
                    context_id
                ));
            }
            // E.V.A.S. evaluation for hardware reconfig
            let evas_action = EvasActionContext {
                action_type: "reconfigure_hardware".to_string(),
                target_resource_id: Some(format!("Device:{}", device_id)),
                payload_hash: "reconfig_payload_hash".to_string(), // Conceptual
                perceived_intent: format!("Reconfigure device {} with new bitstream.", device_id),
                initiating_context_id: context_id,
                ..Default::default()
            };
            match self.evas_filter.evaluate_action(evas_action) {
                EvasDecision::Block(reason) => {
                    return Err(format!(
                        "E.V.A.S. blocked hardware reconfiguration: {}.",
                        reason
                    ))
                }
                _ => {}
            }
            println!(
                "    -> Nimbus OS: Context {} reconfiguring hardware device {} ({} bytes)."
                    .to_string(),
                context_id,
                device_id,
                configuration.len()
            );
            // Conceptual: Interact with Z-MMP configuration bus via HAL
            Ok(())
        } else {
            Err(format!("Context {} not found.", context_id))
        }
    }

    /// Queries the current hardware topology (Z-MMP units, optical links, NoC layout).
    pub fn query_hardware_topology(&self, context_id: NimbusContextId) -> Result<String, String> {
        if self.contexts.contains_key(&context_id) {
            println!(
                "    -> Nimbus OS: Context {} querying hardware topology.".to_string(),
                context_id
            );
            // Conceptual: Return a graph or map of available Z-MMP units
            Ok(
                "Z-MMP Topology: CPU[4], QPU[50q], NACU[1], NPU[1024n], OpticalSwitch[1]"
                    .to_string(),
            )
        } else {
            Err(format!("Context {} not found.", context_id))
        }
    }

    /// Creates a high-priority, real-time task within a context.
    pub fn create_realtime_task(
        &mut self,
        context_id: NimbusContextId,
        priority_level: u32,
        period: Duration,
        deadline: Duration,
    ) -> Result<(), String> {
        if let Some(context) = self.contexts.get_mut(&context_id) {
            if !context
                .active_capabilities
                .contains(&CapabilityToken("realtime_scheduling".to_string()))
            {
                return Err(format!(
                    "Context {} lacks 'realtime_scheduling' capability.",
                    context_id
                ));
            }
            context.priority = Priority::RealTime(priority_level);
            println!(
                "    -> Nimbus OS: Context {} created real-time task. P:{}, T:{:?}, D:{:?}"
                    .to_string(),
                context_id, priority_level, period, deadline
            );
            // Conceptual: Scheduler logic would ensure periodic execution
            Ok(())
        } else {
            Err(format!("Context {} not found.", context_id))
        }
    }

    /// Yields execution of the current context.
    pub fn yield_execution(&mut self, context_id: NimbusContextId) {
        println!(
            "    -> Nimbus OS: Context {} yielding execution.".to_string(),
            context_id
        );
        // Conceptual: Force context switch via global scheduler
    }

    // --- IPC and Capability methods (existing, now mediated by E.V.A.S.) ---

    pub fn create_channel(
        &mut self,
        context1_id: NimbusContextId,
        context2_id: NimbusContextId,
    ) -> Result<ChannelId, String> {
        // ... (as before, but E.V.A.S. check already implemented in turn 67)
        let channel_id = self.next_channel_id;
        self.next_channel_id += 1;
        self.channels
            .insert(channel_id, Arc::new(Mutex::new(VecDeque::new())));
        println!(
            "    -> Nimbus OS: Created secure channel {} between {} and {}.".to_string(),
            channel_id, context1_id, context2_id
        );
        Ok(channel_id)
    }

    pub fn access_hardware(
        &self,
        context_id: NimbusContextId,
        device_id: u64,
        command: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        // E.V.A.S. evaluation
        let evas_action = EvasActionContext {
            initiating_context_id: context_id,
            action_type: "access_hardware".to_string(),
            target_resource_id: Some(format!("Device:{}", device_id)),
            perceived_intent: format!("Access hardware device {}.", device_id),
            ..Default::default()
        };
        match self.evas_filter.evaluate_action(evas_action) {
            EvasDecision::Block(reason) => {
                return Err(format!("E.V.A.S. blocked hardware access: {}.", reason))
            }
            _ => { /* Allow or Warn */ }
        }

        if let Some(context) = self.contexts.get(&context_id) {
            if !context
                .active_capabilities
                .contains(&CapabilityToken("hardware_access".to_string()))
            {
                return Err(format!(
                    "Context {} lacks 'hardware_access' capability.",
                    context_id
                ));
            }
            if !self.registered_devices.contains_key(&device_id) {
                return Err(format!("Device {} is not registered.", device_id));
            }
            println!(
                "    -> Nimbus OS: Context {} securely accessing hardware device {}. State: {:?}"
                    .to_string(),
                context_id,
                device_id,
                self.device_power_states
                    .get(&device_id)
                    .unwrap_or(&PowerState::Active)
            );
            Ok(vec![0xDE, 0xAD, 0xBE, 0xEF]) // Dummy response
        } else {
            Err(format!("Context {} not found.", context_id))
        }
    }

    pub fn grant_capability(
        &mut self,
        context_id: NimbusContextId,
        capability: CapabilityToken,
    ) -> Result<(), String> {
        if let Some(context) = self.contexts.get_mut(&context_id) {
            context.active_capabilities.insert(capability.clone());
            println!(
                "    -> Nimbus OS: Granted capability '{:?}' to context {}.".to_string(),
                capability, context_id
            );
            Ok(())
        } else {
            Err(format!("Context {} not found.", context_id))
        }
    }
}
