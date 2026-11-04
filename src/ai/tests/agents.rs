
#![forbid(unsafe_code)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]

//! # Zamani AI Agent Subsystem Tests
//!
//! Path: src/ai/tests/agents.rs
//!
//! Rust edition: 2021
//! Minimum Rust version: 1.97
//!
//! ## Purpose
//!
//! Production-oriented contract and integration tests for the
//! existing Zamani AI agent subsystem.
//!
//! ## Tested authorities
//!
//! - crate::ai::ids
//! - crate::ai::agents::definition
//! - crate::ai::agents::capabilities
//! - crate::ai::agents::communication
//! - crate::ai::agents::coordination
//! - crate::ai::agents::planning
//! - crate::ai::agents::memory
//! - crate::ai::agents::tools
//! - crate::ai::agents::policies
//!
//! ## Architectural boundary
//!
//! These tests verify declarative agent computation.
//!
//! They do not pretend that constructing an agent definition
//! executes an agent or establishes hardware feasibility.
//!
//! Source parsing, AST construction, semantic validation,
//! transport, authorization, scheduling and runtime execution
//! remain owned by their respective repository subsystems.
//!
//! ## Scalability
//!
//! Tests must not establish artificial language-level limits
//! on agent counts, participants, communication endpoints,
//! planning steps, memory declarations or tool definitions.
//!
//! Practical test workloads are test parameters, not semantic
//! capacity ceilings.

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet, HashSet};
    use std::sync::Arc;
    use std::thread;

    use serde_json::json;

    use crate::ai::ids::AiAgentId;

    use crate::ai::agents::{
        AgentDefinition,
        AgentDefinitionError,
        AgentName,
        AgentResponsibility,
        AgentInterface,
        AgentInterfacePort,
        AgentPortDirection,
        AgentLifecycle,
        AgentLifecycleState,
        AgentLifecycleTransition,
        QualifiedAgentName,

        AgentCapabilityProfile,
        AgentCapabilitiesError,

        AgentCommunicationProfile,
        AgentCommunicationEndpoint,
        AgentCommunicationChannel,
        AgentCommunicationStream,
        AgentMessageContract,
        AgentCommunicationName,
        AgentCommunicationError,
        CommunicationTypeRef,
        CommunicationReference,
        CommunicationValueRef,
        EndpointDirection,
        DeliveryGuarantee,
        OrderingGuarantee,

        AgentCoordinationGroup,
        AgentCoordinationPlan,
        AgentCoordinationRelationship,
        AgentParticipation,
        CoordinationName,
        CoordinationStrategy,
        CoordinationRole,
        CoordinationRelationshipKind,
        ParticipationKind,
        ParticipationRequirement,

        AgentPlan,
        PlanningName,
        PlanningReference,
        PlanningStep,
        PlanningStepId,
        PlanningDependency,
        PlanningDependencyKind,
        AgentPlanningError,

        AgentMemoryProfile,
        AgentMemoryDeclaration,
        AgentMemoryName,
        AgentMemoryKind,
        AgentMemoryReference,
        AgentMemoryValueReference,
        AgentMemoryAccess,
        AgentMemoryOperation,
        AgentMemoryRequest,
        AgentMemoryError,

        AgentToolDefinition,
        AgentToolName,
        AgentToolReference,
        AgentToolProviderReference,
        AgentToolVersion,
        AgentToolSchema,
        AgentToolParameter,
        AgentToolValueType,
        AgentToolInvocation,
        AgentToolInvocationId,
        AgentToolResponse,
        AgentToolProvenanceReference,
        AgentToolAuthorizationReference,

        AgentPolicyReference,
        AgentPolicyBinding,
        AgentPolicyBindingId,
        AgentPolicySubject,
        AgentPolicySet,
        AgentPolicyError,
    };

    // ================================================================
    // TEST HELPERS
    // ================================================================

    fn agent_id(value: u128) -> AiAgentId {
        AiAgentId::new(value)
            .expect("agent identity must be valid")
    }

    fn qualified_agent_name(
        value: &str,
    ) -> QualifiedAgentName {
        QualifiedAgentName::new(
            Vec::new(),
            AgentName::new(value)
                .expect("valid agent name"),
        )
        .expect("valid qualified agent name")
    }

    fn coordination_name(
        value: &str,
    ) -> CoordinationName {
        CoordinationName::new(value)
            .expect("valid coordination name")
    }

    fn planning_name(
        value: &str,
    ) -> PlanningName {
        PlanningName::new(value)
            .expect("valid planning name")
    }

    fn planning_reference(
        value: &str,
    ) -> PlanningReference {
        PlanningReference::new(value)
            .expect("valid planning reference")
    }

    fn planning_step(
        value: &str,
    ) -> PlanningStep {
        PlanningStep::new(
            PlanningStepId::new(value)
                .expect("valid planning step ID"),
            planning_reference(
                format!("operation::{value}")
                    .as_str(),
            ),
        )
    }

    fn planning_dependency(
        from: &str,
        to: &str,
    ) -> PlanningDependency {
        PlanningDependency::new(
            PlanningStepId::new(from)
                .expect("valid source"),
            PlanningStepId::new(to)
                .expect("valid target"),
            PlanningDependencyKind::Requires,
        )
        .expect("valid dependency")
    }

    fn memory_name(
        value: &str,
    ) -> AgentMemoryName {
        AgentMemoryName::new(value)
            .expect("valid memory name")
    }

    fn memory_kind(
        value: &str,
    ) -> AgentMemoryKind {
        AgentMemoryKind::new(value)
            .expect("valid memory kind")
    }

    fn memory_reference(
        value: &str,
    ) -> AgentMemoryReference {
        AgentMemoryReference::new(value)
            .expect("valid memory reference")
    }

    fn policy_binding(
        id: &str,
    ) -> AgentPolicyBinding {
        AgentPolicyBinding::new(
            AgentPolicyBindingId::new(id)
                .expect("valid binding identity"),
            AgentPolicyReference::new(
                "zamani.agent",
                "execution",
            )
            .expect("valid policy reference"),
            AgentPolicySubject::AllAgents,
        )
        .expect("valid policy binding")
    }

    // ================================================================
    // 1. AGENT IDENTITY AND DEFINITIONS
    // ================================================================

    #[test]
    fn agent_identity_supports_full_u128_range() {
        let smallest = agent_id(1);
        let largest = agent_id(u128::MAX);

        assert_eq!(smallest.get(), 1);
        assert_eq!(largest.get(), u128::MAX);
    }

    #[test]
    fn zero_agent_identity_is_rejected() {
        assert!(AiAgentId::new(0).is_err());
    }

    #[test]
    fn agent_names_support_open_world_extensions() {
        let name = AgentName::new(
            "future_neuro_symbolic_quantum_agent",
        )
        .expect("valid extensible name");

        assert_eq!(
            name.display_name(),
            "future_neuro_symbolic_quantum_agent"
        );
    }

    #[test]
    fn invalid_agent_names_are_rejected() {
        assert!(AgentName::new("").is_err());

        assert!(AgentName::new(" Researcher").is_err());

        assert!(AgentName::new("Researcher ").is_err());
    }

    #[test]
    fn qualified_agent_names_preserve_namespaces() {
        let name = QualifiedAgentName::new(
            vec![
                "research".to_owned(),
                "distributed".to_owned(),
                "quantum".to_owned(),
            ],
            AgentName::new("Planner")
                .expect("valid name"),
        )
        .expect("valid qualified name");

        assert_eq!(
            name.display_name(),
            "research::distributed::quantum::Planner"
        );
    }

    #[test]
    fn agent_definitions_preserve_identity() {
        let definition = AgentDefinition::new(
            agent_id(42),
            qualified_agent_name("Researcher"),
        );

        assert_eq!(definition.id(), agent_id(42));

        assert_eq!(
            definition.name().display_name(),
            "Researcher"
        );
    }

    #[test]
    fn agent_responsibilities_are_extensible() {
        let mut definition = AgentDefinition::new(
            agent_id(1),
            qualified_agent_name("Researcher"),
        );

        for responsibility in [
            "research",
            "reasoning",
            "planning",
            "quantum_assisted_analysis",
            "future_computational_domain",
        ] {
            definition.add_responsibility(
                AgentResponsibility::new(responsibility)
                    .expect("valid responsibility"),
            );
        }

        assert_eq!(
            definition.responsibilities().len(),
            5
        );

        definition.validate()
            .expect("valid agent definition");
    }

    #[test]
    fn duplicate_responsibilities_are_idempotent() {
        let mut definition = AgentDefinition::new(
            agent_id(2),
            qualified_agent_name("Researcher"),
        );

        let responsibility =
            AgentResponsibility::new("research")
                .expect("valid responsibility");

        definition.add_responsibility(
            responsibility.clone(),
        );

        definition.add_responsibility(
            responsibility,
        );

        assert_eq!(
            definition.responsibilities().len(),
            1
        );
    }

    #[test]
    fn agent_interfaces_validate_input_and_output_ports() {
        let input = AgentInterfacePort::new(
            "query",
            "String",
            AgentPortDirection::Input,
        )
        .expect("valid input");

        let output = AgentInterfacePort::new(
            "answer",
            "String",
            AgentPortDirection::Output,
        )
        .expect("valid output");

        let interface = AgentInterface::from_ports(
            vec![input, output],
        )
        .expect("valid interface");

        assert_eq!(interface.inputs().count(), 1);
        assert_eq!(interface.outputs().count(), 1);
    }

    #[test]
    fn duplicate_interface_ports_are_rejected() {
        let mut interface = AgentInterface::new();

        interface.add_port(
            AgentInterfacePort::new(
                "value",
                "Integer",
                AgentPortDirection::Input,
            )
            .expect("valid port"),
        )
        .expect("first insertion");

        let result = interface.add_port(
            AgentInterfacePort::new(
                "value",
                "Integer",
                AgentPortDirection::Output,
            )
            .expect("valid port"),
        );

        assert!(matches!(
            result,
            Err(
                AgentDefinitionError::DuplicateInterfacePort(_)
            )
        ));
    }

    #[test]
    fn optional_agent_interface_ports_are_supported() {
        let port = AgentInterfacePort::new(
            "context",
            "Context",
            AgentPortDirection::Input,
        )
        .expect("valid port")
        .optional();

        assert!(port.is_optional());
    }

    #[test]
    fn agent_lifecycle_validates_state_transitions() {
        let mut lifecycle = AgentLifecycle::new();

        let idle = AgentLifecycleState::new("idle")
            .expect("valid state");

        let active = AgentLifecycleState::new("active")
            .expect("valid state");

        lifecycle.add_state(idle.clone())
            .expect("idle state");

        lifecycle.add_state(active.clone())
            .expect("active state");

        lifecycle.set_initial_state(idle.clone())
            .expect("initial state");

        lifecycle.add_transition(
            AgentLifecycleTransition::new(
                idle,
                active,
            )
            .expect("valid transition"),
        )
        .expect("transition insertion");

        lifecycle.validate()
            .expect("valid lifecycle");
    }

    #[test]
    fn duplicate_lifecycle_states_are_rejected() {
        let mut lifecycle = AgentLifecycle::new();

        let state = AgentLifecycleState::new("idle")
            .expect("valid state");

        lifecycle.add_state(state.clone())
            .expect("first insertion");

        assert!(lifecycle.add_state(state).is_err());
    }

    #[test]
    fn invalid_self_transitions_are_rejected() {
        let state = AgentLifecycleState::new("idle")
            .expect("valid state");

        assert!(
            AgentLifecycleTransition::new(
                state.clone(),
                state,
            )
            .is_err()
        );
    }

    #[test]
    fn agent_definitions_support_serialization_round_trip() {
        let mut definition = AgentDefinition::new(
            agent_id(42),
            qualified_agent_name("Researcher"),
        );

        definition.add_responsibility(
            AgentResponsibility::new("research")
                .expect("valid responsibility"),
        );

        let encoded = serde_json::to_string(&definition)
            .expect("serialize definition");

        let decoded: AgentDefinition =
            serde_json::from_str(&encoded)
                .expect("deserialize definition");

        assert_eq!(decoded, definition);
    }

    // ================================================================
    // 2. AGENT CAPABILITIES
    // ================================================================

    #[test]
    fn capability_profiles_use_canonical_agent_identity() {
        let profile =
            AgentCapabilityProfile::new(agent_id(10));

        assert_eq!(profile.agent_id(), agent_id(10));

        assert!(profile.is_empty());

        assert!(profile.is_analysis_complete());
    }

    #[test]
    fn capability_profiles_support_open_world_capabilities() {
        let mut profile =
            AgentCapabilityProfile::new(agent_id(11));

        for capability in [
            "inference",
            "reasoning",
            "planning",
            "quantum_machine_learning",
            "future_computation",
        ] {
            let descriptor =
                crate::ai::capabilities::AiCapabilityDescriptor::for_ai(
                    capability,
                    crate::ai::capabilities::AiCapabilityCategory::new(
                        "agent_computation",
                    )
                    .expect("valid category"),
                    "Agent computational capability",
                )
                .expect("valid capability");

            profile.add_offer(descriptor)
                .expect("capability offer");
        }

        assert_eq!(profile.offers().len(), 5);
    }

    #[test]
    fn duplicate_capability_offers_are_rejected() {
        let mut profile =
            AgentCapabilityProfile::new(agent_id(12));

        let descriptor = || {
            crate::ai::capabilities::AiCapabilityDescriptor::for_ai(
                "inference",
                crate::ai::capabilities::AiCapabilityCategory::new(
                    "agent_computation",
                )
                .expect("valid category"),
                "Inference capability",
            )
            .expect("valid descriptor")
        };

        profile.add_offer(descriptor())
            .expect("first offer");

        assert!(matches!(
            profile.add_offer(descriptor()),
            Err(AgentCapabilitiesError::DuplicateOffer { .. })
        ));
    }

    #[test]
    fn capability_requirement_strengths_remain_distinct() {
        use crate::ai::requirements::CapabilityStrength;

        let mut profile =
            AgentCapabilityProfile::new(agent_id(13));

        profile.require_ai_capability(
            "inference",
            CapabilityStrength::Required,
        )
        .expect("required capability");

        profile.require_ai_capability(
            "accelerated_inference",
            CapabilityStrength::Preferred,
        )
        .expect("preferred capability");

        profile.require_ai_capability(
            "distributed_execution",
            CapabilityStrength::Allowed,
        )
        .expect("allowed capability");

        profile.require_ai_capability(
            "untrusted_execution",
            CapabilityStrength::Forbidden,
        )
        .expect("forbidden capability");

        let requirements =
            profile.requirements().capabilities();

        assert_eq!(requirements.len(), 4);

        let strengths: BTreeSet<_> = requirements
            .iter()
            .map(|requirement| requirement.strength())
            .collect();

        assert_eq!(strengths.len(), 4);
    }

    // ================================================================
    // 3. COMMUNICATION CONTRACTS
    // ================================================================

    #[test]
    fn communication_profiles_are_agent_scoped() {
        let profile =
            AgentCommunicationProfile::new(agent_id(20));

        assert_eq!(profile.agent_id(), agent_id(20));

        assert!(profile.is_empty());

        assert!(profile.validate().is_ok());
    }

    #[test]
    fn communication_names_are_open_world() {
        let name = AgentCommunicationName::new(
            "future.quantum.agent.communication",
        )
        .expect("valid name");

        assert_eq!(
            name.as_str(),
            "future.quantum.agent.communication"
        );
    }

    #[test]
    fn invalid_communication_names_are_rejected() {
        assert!(AgentCommunicationName::new("").is_err());

        assert!(
            AgentCommunicationName::new(" channel ").is_err()
        );
    }

    #[test]
    fn communication_profiles_support_multiple_endpoints() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(21));

        for endpoint_name in [
            "input",
            "output",
            "control",
            "telemetry",
        ] {
            profile.add_endpoint(
                AgentCommunicationEndpoint::new(
                    AgentCommunicationName::new(endpoint_name)
                        .expect("valid endpoint"),
                    EndpointDirection::Bidirectional,
                    None,
                ),
            )
            .expect("endpoint insertion");
        }

        assert_eq!(profile.endpoints().len(), 4);
    }

    #[test]
    fn duplicate_communication_endpoints_are_rejected() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(22));

        let endpoint = || {
            AgentCommunicationEndpoint::new(
                AgentCommunicationName::new("inbox")
                    .expect("valid name"),
                EndpointDirection::Inbound,
                None,
            )
        };

        profile.add_endpoint(endpoint())
            .expect("first endpoint");

        assert!(matches!(
            profile.add_endpoint(endpoint()),
            Err(AgentCommunicationError::DuplicateEndpoint(_))
        ));
    }

    #[test]
    fn channels_and_streams_are_independent_contracts() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(23));

        profile.add_channel(
            AgentCommunicationChannel::new(
                AgentCommunicationName::new("research")
                    .expect("valid channel"),
                None,
            ),
        )
        .expect("channel insertion");

        profile.add_stream(
            AgentCommunicationStream::new(
                AgentCommunicationName::new("observations")
                    .expect("valid stream"),
                None,
            ),
        )
        .expect("stream insertion");

        assert_eq!(profile.channels().len(), 1);
        assert_eq!(profile.streams().len(), 1);
    }

    #[test]
    fn message_contracts_preserve_delivery_and_ordering_semantics() {
        let mut profile =
            AgentCommunicationProfile::new(agent_id(24));

        let message = AgentMessageContract::new(
            AgentCommunicationName::new("research_result")
                .expect("valid message"),
            Some(
                CommunicationTypeRef::new(
                    "zamani::ResearchResult",
                )
                .expect("valid type"),
            ),
        )
        .with_delivery(DeliveryGuarantee::AtLeastOnce)
        .with_ordering(OrderingGuarantee::PerChannel);

        profile.add_message_contract(message)
            .expect("message insertion");

        assert_eq!(profile.messages().len(), 1);
    }

    #[test]
    fn message_invocations_preserve_argument_order() {
        let invocation =
            crate::ai::agents::AgentMessageInvocation::new(
                agent_id(25),
                CommunicationReference::new("research_agent")
                    .expect("valid target"),
                vec![
                    CommunicationValueRef::new("first")
                        .expect("first argument"),
                    CommunicationValueRef::new("second")
                        .expect("second argument"),
                ],
            );

        assert_eq!(invocation.arguments().len(), 2);

        assert_eq!(
            invocation.arguments()[0].as_str(),
            "first"
        );

        assert_eq!(
            invocation.arguments()[1].as_str(),
            "second"
        );
    }

    #[test]
    fn communication_profiles_cannot_merge_different_agents() {
        let mut first =
            AgentCommunicationProfile::new(agent_id(26));

        let second =
            AgentCommunicationProfile::new(agent_id(27));

        assert!(matches!(
            first.merge(&second),
            Err(AgentCommunicationError::AgentMismatch { .. })
        ));

        assert!(first.is_empty());
    }

    // ================================================================
    // 4. MULTI-AGENT COORDINATION
    // ================================================================

    fn coordination_group(
        name: &str,
        participants: &[u128],
    ) -> AgentCoordinationGroup {
        let mut group = AgentCoordinationGroup::new(
            coordination_name(name),
            CoordinationStrategy::new(
                coordination_name("collaborative"),
            ),
        );

        for participant in participants {
            group.add_participant(
                AgentParticipation::required(
                    agent_id(*participant),
                ),
            )
            .expect("participant insertion");
        }

        group
    }

    #[test]
    fn coordination_supports_extensible_strategies() {
        let strategy = CoordinationStrategy::new(
            coordination_name(
                "future.quantum_neuro_symbolic_consensus",
            ),
        );

        assert_eq!(
            strategy.name().as_str(),
            "future.quantum_neuro_symbolic_consensus"
        );
    }

    #[test]
    fn coordination_groups_support_heterogeneous_agents() {
        let mut group = coordination_group(
            "hybrid_research",
            &[30, 31, 32],
        );

        group.add_participant(
            AgentParticipation::optional(agent_id(33)),
        )
        .expect("optional participant");

        assert_eq!(group.participants().len(), 4);

        assert_eq!(
            group.find_participant(agent_id(33))
                .expect("participant")
                .kind(),
            ParticipationKind::Optional
        );

        group.validate()
            .expect("valid coordination group");
    }

    #[test]
    fn duplicate_coordination_participants_are_rejected() {
        let mut group =
            coordination_group("research", &[34]);

        assert!(
            group.add_participant(
                AgentParticipation::required(agent_id(34))
            )
            .is_err()
        );
    }

    #[test]
    fn coordination_rejects_self_relationships() {
        assert!(
            AgentCoordinationRelationship::new(
                agent_id(35),
                agent_id(35),
                CoordinationRelationshipKind::new(
                    coordination_name("delegation"),
                ),
            )
            .is_err()
        );
    }

    #[test]
    fn coordination_relationships_require_known_participants() {
        let mut group =
            coordination_group("research", &[36, 37]);

        let relationship =
            AgentCoordinationRelationship::new(
                agent_id(36),
                agent_id(37),
                CoordinationRelationshipKind::new(
                    coordination_name("delegation"),
                ),
            )
            .expect("valid relationship");

        group.add_relationship(relationship)
            .expect("valid relationship insertion");

        assert!(group.validate().is_ok());
    }

    #[test]
    fn coordination_supports_custom_participation_requirements() {
        let group = coordination_group(
            "adaptive_group",
            &[38, 39],
        )
        .with_participation_requirement(
            ParticipationRequirement::Custom(
                coordination_name("weighted_consensus"),
            ),
        );

        assert!(group.validate().is_ok());
    }

    #[test]
    fn coordination_plans_support_multiple_groups() {
        let mut plan = AgentCoordinationPlan::new();

        plan.add_group(
            coordination_group("research", &[40, 41]),
        )
        .expect("research group");

        plan.add_group(
            coordination_group("verification", &[41, 42]),
        )
        .expect("verification group");

        assert_eq!(plan.len(), 2);
        assert_eq!(plan.participant_count(), 3);
    }

    // ================================================================
    // 5. AGENT PLANNING
    // ================================================================

    #[test]
    fn planning_supports_empty_valid_plans() {
        let plan =
            AgentPlan::new(planning_name("research"));

        assert!(plan.validate().is_ok());
        assert!(plan.has_no_steps());
    }

    #[test]
    fn planning_supports_extensible_steps() {
        let mut plan =
            AgentPlan::new(planning_name("research"));

        for name in [
            "collect",
            "analyze",
            "reason",
            "verify",
            "publish",
        ] {
            plan.add_step(planning_step(name));
        }

        assert_eq!(plan.step_count(), 5);

        assert!(plan.validate().is_ok());
    }

    #[test]
    fn duplicate_planning_steps_are_rejected() {
        let mut plan =
            AgentPlan::new(planning_name("research"));

        plan.add_step(planning_step("collect"));

        assert!(matches!(
            plan.add_step(planning_step("collect")),
            Err(AgentPlanningError::DuplicateStep(_))
        ));
    }

    #[test]
    fn planning_rejects_unknown_dependency_endpoints() {
        let mut plan =
            AgentPlan::new(planning_name("research"));

        plan.add_step(planning_step("collect"));

        assert!(matches!(
            plan.add_dependency(
                planning_dependency("unknown", "collect")
            ),
            Err(AgentPlanningError::UnknownStep(_))
        ));
    }

    #[test]
    fn planning_rejects_dependency_cycles() {
        let mut plan =
            AgentPlan::new(planning_name("research"));

        for name in ["a", "b", "c"] {
            plan.add_step(planning_step(name));
        }

        plan.add_dependency(planning_dependency("a", "b"))
            .expect("first dependency");

        plan.add_dependency(planning_dependency("b", "c"))
            .expect("second dependency");

        plan.add_dependency(planning_dependency("c", "a"))
            .expect("third dependency");

        assert!(plan.validate().is_err());
    }

    #[test]
    fn planning_topological_order_is_deterministic() {
        let mut plan =
            AgentPlan::new(planning_name("research"));

        for name in ["zeta", "alpha", "middle"] {
            plan.add_step(planning_step(name));
        }

        let order = plan.topological_order()
            .expect("valid ordering");

        let names: Vec<_> = order
            .iter()
            .map(|step| step.as_str())
            .collect();

        assert_eq!(
            names,
            vec!["alpha", "middle", "zeta"]
        );
    }

    #[test]
    fn planning_dependencies_preserve_execution_constraints() {
        let mut plan =
            AgentPlan::new(planning_name("research"));

        for name in ["collect", "analyze", "report"] {
            plan.add_step(planning_step(name));
        }

        plan.add_dependency(
            planning_dependency("collect", "analyze"),
        )
        .expect("dependency");

        plan.add_dependency(
            planning_dependency("analyze", "report"),
        )
        .expect("dependency");

        let order = plan.topological_order()
            .expect("valid ordering");

        let positions: BTreeMap<_, _> = order
            .iter()
            .enumerate()
            .map(|(index, id)| (id.as_str(), index))
            .collect();

        assert!(
            positions["collect"] < positions["analyze"]
        );

        assert!(
            positions["analyze"] < positions["report"]
        );
    }

    // ================================================================
    // 6. AGENT MEMORY
    // ================================================================

    #[test]
    fn memory_profiles_are_agent_scoped() {
        let profile =
            AgentMemoryProfile::new(agent_id(50));

        assert_eq!(profile.agent_id(), agent_id(50));

        assert!(profile.is_empty());

        assert!(profile.validate().is_ok());
    }

    #[test]
    fn memory_declarations_support_open_world_categories() {
        let declaration = AgentMemoryDeclaration::new(
            memory_name("research_state"),
            memory_kind("vendor.extension.temporal_graph"),
            memory_reference("provider://research/state"),
        )
        .with_access(AgentMemoryAccess::Read)
        .with_access(AgentMemoryAccess::Write);

        assert_eq!(
            declaration.kind().as_str(),
            "vendor.extension.temporal_graph"
        );

        assert!(declaration.access().contains(
            &AgentMemoryAccess::Read
        ));

        assert!(declaration.access().contains(
            &AgentMemoryAccess::Write
        ));

        declaration.validate()
            .expect("valid declaration");
    }

    #[test]
    fn memory_profiles_reject_duplicate_declarations() {
        let declaration = AgentMemoryDeclaration::new(
            memory_name("state"),
            memory_kind("working"),
            memory_reference("memory://state"),
        );

        let mut profile =
            AgentMemoryProfile::new(agent_id(51));

        profile.add_declaration(declaration.clone())
            .expect("first declaration");

        assert!(matches!(
            profile.add_declaration(declaration),
            Err(AgentMemoryError::DuplicateDeclaration { .. })
        ));

        assert_eq!(profile.len(), 1);
    }

    #[test]
    fn memory_profiles_support_lookup_and_removal() {
        let declaration = AgentMemoryDeclaration::new(
            memory_name("history"),
            memory_kind("episodic"),
            memory_reference("memory://history"),
        );

        let mut profile =
            AgentMemoryProfile::new(agent_id(52));

        profile.add_declaration(declaration)
            .expect("declaration");

        let name = memory_name("history");

        assert!(profile.find_declaration(&name).is_some());

        assert!(profile.remove_declaration(&name).is_some());

        assert!(profile.find_declaration(&name).is_none());
    }

    #[test]
    fn memory_read_requests_require_target_references() {
        let request = AgentMemoryRequest::new(
            agent_id(53),
            AgentMemoryOperation::Read,
            memory_reference("memory://state"),
        );

        assert!(matches!(
            request.validate(),
            Err(AgentMemoryError::MissingTargetReference { .. })
        ));
    }

    #[test]
    fn memory_read_requests_accept_target_references() {
        let request = AgentMemoryRequest::new(
            agent_id(54),
            AgentMemoryOperation::Read,
            memory_reference("memory://state"),
        )
        .with_target(
            AgentMemoryValueReference::new("record:42")
                .expect("valid target"),
        );

        assert!(request.validate().is_ok());
    }

    #[test]
    fn memory_write_requests_require_values() {
        let request = AgentMemoryRequest::new(
            agent_id(55),
            AgentMemoryOperation::Write,
            memory_reference("memory://state"),
        );

        assert_eq!(
            request.validate(),
            Err(AgentMemoryError::MissingValueReference)
        );
    }

    #[test]
    fn memory_write_requests_accept_values() {
        let request = AgentMemoryRequest::new(
            agent_id(56),
            AgentMemoryOperation::Write,
            memory_reference("memory://state"),
        )
        .with_value(
            AgentMemoryValueReference::new("value:42")
                .expect("valid value"),
        );

        assert!(request.validate().is_ok());
    }

    // ================================================================
    // 7. AGENT TOOLS
    // ================================================================

    fn tool_definition() -> AgentToolDefinition {
        let mut input = AgentToolSchema::new();

        input.add_parameter(
            AgentToolParameter::new(
                "query",
                AgentToolValueType::String,
                true,
            )
            .expect("valid parameter"),
        )
        .expect("parameter insertion");

        input.add_parameter(
            AgentToolParameter::new(
                "limit",
                AgentToolValueType::Integer,
                false,
            )
            .expect("valid parameter"),
        )
        .expect("parameter insertion");

        let mut output = AgentToolSchema::new();

        output.add_parameter(
            AgentToolParameter::new(
                "result",
                AgentToolValueType::String,
                true,
            )
            .expect("valid parameter"),
        )
        .expect("parameter insertion");

        AgentToolDefinition::new(
            AgentToolReference::new("tool.search")
                .expect("valid reference"),
            AgentToolName::new("search")
                .expect("valid name"),
            AgentToolVersion::new(1, 0, 0),
            AgentToolProviderReference::new("provider.local")
                .expect("valid provider"),
            "Search provider",
        )
        .expect("valid tool")
        .with_input_schema(input)
        .with_output_schema(output)
    }

    #[test]
    fn tool_schemas_validate_required_arguments() {
        let definition = tool_definition();

        assert!(
            definition.input_schema().validate_value(
                &json!({"query": "Zamani"})
            ).is_ok()
        );

        assert!(
            definition.input_schema().validate_value(
                &json!({})
            ).is_err()
        );
    }

    #[test]
    fn tool_schemas_reject_wrong_argument_types() {
        let definition = tool_definition();

        assert!(
            definition.input_schema().validate_value(
                &json!({"query": 42})
            ).is_err()
        );
    }

    #[test]
    fn tool_schemas_reject_undeclared_arguments_by_default() {
        let definition = tool_definition();

        assert!(
            definition.input_schema().validate_value(
                &json!({
                    "query": "Zamani",
                    "unexpected": true
                })
            ).is_err()
        );
    }

    #[test]
    fn tool_invocations_must_match_their_definitions() {
        let definition = tool_definition();

        let invocation = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.1")
                .expect("valid invocation ID"),
            agent_id(60),
            AgentToolReference::new("tool.other")
                .expect("valid tool reference"),
            json!({"query": "Zamani"}),
        )
        .expect("valid invocation");

        assert!(
            invocation.validate_against(&definition).is_err()
        );
    }

    #[test]
    fn valid_tool_invocations_pass_schema_validation() {
        let definition = tool_definition();

        let invocation = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.2")
                .expect("valid invocation ID"),
            agent_id(61),
            AgentToolReference::new("tool.search")
                .expect("valid tool reference"),
            json!({
                "query": "Zamani",
                "limit": 10
            }),
        )
        .expect("valid invocation");

        invocation.validate_against(&definition)
            .expect("valid invocation");
    }

    #[test]
    fn tool_responses_must_match_invocation_identity() {
        let definition = tool_definition();

        let invocation = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.3")
                .expect("valid ID"),
            agent_id(62),
            AgentToolReference::new("tool.search")
                .expect("valid reference"),
            json!({"query": "Zamani"}),
        )
        .expect("valid invocation");

        let response = AgentToolResponse::new(
            AgentToolInvocationId::new("invocation.other")
                .expect("valid ID"),
            json!({"result": "answer"}),
        )
        .expect("valid response");

        assert!(
            response.validate_against(
                &invocation,
                &definition,
            ).is_err()
        );
    }

    #[test]
    fn valid_tool_responses_pass_output_validation() {
        let definition = tool_definition();

        let invocation = AgentToolInvocation::new(
            AgentToolInvocationId::new("invocation.4")
                .expect("valid ID"),
            agent_id(63),
            AgentToolReference::new("tool.search")
                .expect("valid reference"),
            json!({"query": "Zamani"}),
        )
        .expect("valid invocation");

        let response = AgentToolResponse::new(
            invocation.id().clone(),
            json!({"result": "answer"}),
        )
        .expect("valid response");

        response.validate_against(
            &invocation,
            &definition,
        )
        .expect("valid response");
    }

    #[test]
    fn tool_provenance_and_authorization_references_are_preserved() {
        let mut definition = tool_definition()
            .with_provenance(
                AgentToolProvenanceReference::new(
                    "provenance.tool.search",
                )
                .expect("valid provenance"),
            );

        definition.add_authorization_requirement(
            AgentToolAuthorizationReference::new(
                "security.tool.search",
            )
            .expect("valid authorization reference"),
        );

        assert_eq!(
            definition.provenance()
                .expect("provenance")
                .as_str(),
            "provenance.tool.search"
        );

        assert_eq!(
            definition.authorization_requirements().len(),
            1
        );
    }

    // ================================================================
    // 8. AGENT POLICIES
    // ================================================================

    #[test]
    fn agent_policy_references_are_validated() {
        assert!(
            AgentPolicyReference::new(
                "zamani.agent",
                "execution",
            ).is_ok()
        );

        assert!(
            AgentPolicyReference::new("", "execution").is_err()
        );

        assert!(
            AgentPolicyReference::new("zamani.agent", "").is_err()
        );
    }

    #[test]
    fn policy_bindings_preserve_explicit_scope() {
        let binding = policy_binding("execution_binding");

        assert_eq!(
            binding.id().as_str(),
            "execution_binding"
        );

        assert_eq!(
            binding.policy().name(),
            "execution"
        );

        assert!(
            binding.validate().is_ok()
        );
    }

    #[test]
    fn policy_sets_reject_duplicate_binding_identities() {
        let mut policies = AgentPolicySet::new();

        policies.attach(policy_binding("first"))
            .expect("first binding");

        assert!(matches!(
            policies.attach(policy_binding("first")),
            Err(AgentPolicyError::DuplicateBinding { .. })
        ));
    }

    #[test]
    fn policy_sets_support_multiple_bindings() {
        let mut policies = AgentPolicySet::new();

        policies.attach(policy_binding("first"))
            .expect("first");

        policies.attach(policy_binding("second"))
            .expect("second");

        assert_eq!(policies.len(), 2);

        assert!(policies.validate().is_ok());
    }

    #[test]
    fn policy_binding_order_is_deterministic() {
        let mut policies = AgentPolicySet::new();

        for name in ["zeta", "alpha", "middle"] {
            policies.attach(policy_binding(name))
                .expect("valid binding");
        }

        let names: Vec<_> = policies
            .iter()
            .map(|binding| binding.id().as_str())
            .collect();

        assert_eq!(
            names,
            vec!["alpha", "middle", "zeta"]
        );
    }

    #[test]
    fn policy_sets_support_detaching_bindings() {
        let mut policies = AgentPolicySet::new();

        let binding = policy_binding("removable");
        let id = binding.id().clone();

        policies.attach(binding)
            .expect("valid binding");

        assert!(policies.get(&id).is_some());

        assert!(policies.detach(&id).is_some());

        assert!(policies.get(&id).is_none());
    }

    // ================================================================
    // 9. CROSS-COMPONENT INTEGRATION
    // ================================================================

    #[test]
    fn complete_agent_contracts_compose_without_owning_execution() {
        let identity = agent_id(100);

        // Agent definition.
        let mut definition = AgentDefinition::new(
            identity,
            qualified_agent_name("Researcher"),
        );

        definition.add_responsibility(
            AgentResponsibility::new("research")
                .expect("responsibility"),
        );

        definition.validate()
            .expect("definition validation");

        // Capability profile.
        let capabilities =
            AgentCapabilityProfile::new(identity);

        assert_eq!(
            capabilities.agent_id(),
            definition.id()
        );

        // Communication profile.
        let communication =
            AgentCommunicationProfile::new(identity);

        assert_eq!(
            communication.agent_id(),
            definition.id()
        );

        // Memory profile.
        let memory =
            AgentMemoryProfile::new(identity);

        assert_eq!(
            memory.agent_id(),
            definition.id()
        );

        // Planning.
        let mut plan =
            AgentPlan::new(planning_name("research"));

        plan.add_step(planning_step("collect"));
        plan.add_step(planning_step("analyze"));

        plan.add_dependency(
            planning_dependency("collect", "analyze"),
        )
        .expect("valid dependency");

        assert!(plan.validate().is_ok());

        // Policy attachments.
        let policies = AgentPolicySet::from_bindings([
            policy_binding("execution"),
        ])
        .expect("valid policies");

        assert!(policies.validate().is_ok());

        // The test establishes consistency between declarative
        // contracts. It does not claim that the agent was executed.
        assert_eq!(definition.id(), identity);
        assert_eq!(capabilities.agent_id(), identity);
        assert_eq!(communication.agent_id(), identity);
        assert_eq!(memory.agent_id(), identity);
    }

    #[test]
    fn agent_contract_serialization_preserves_structural_identity() {
        let original = AgentDefinition::new(
            agent_id(101),
            qualified_agent_name("Planner"),
        );

        let serialized = serde_json::to_string(&original)
            .expect("serialization");

        let restored: AgentDefinition =
            serde_json::from_str(&serialized)
                .expect("deserialization");

        assert_eq!(original, restored);
    }

    #[test]
    fn independent_agent_identities_remain_distinct() {
        let identities: HashSet<_> = (1..=100)
            .map(agent_id)
            .collect();

        assert_eq!(identities.len(), 100);
    }

    // ================================================================
    // 10. CONCURRENCY AND SCALABILITY
    // ================================================================

    #[test]
    fn agent_identifiers_are_safe_for_concurrent_allocation() {
        let allocator = Arc::new(
            crate::ai::ids::IdAllocator::<AiAgentId>::new(),
        );

        let workers = thread::available_parallelism()
            .map(|value| value.get())
            .unwrap_or(1);

        let mut handles = Vec::new();

        for _ in 0..workers {
            let allocator = Arc::clone(&allocator);

            handles.push(thread::spawn(move || {
                let mut identities = Vec::new();

                for _ in 0..128 {
                    identities.push(
                        allocator.try_next()
                            .expect("identity allocation"),
                    );
                }

                identities
            }));
        }

        let mut identities = Vec::new();

        for handle in handles {
            identities.extend(
                handle.join()
                    .expect("allocation worker completed"),
            );
        }

        let unique: HashSet<_> =
            identities.iter().copied().collect();

        assert_eq!(
            unique.len(),
            identities.len()
        );
    }

    #[test]
    fn agent_contracts_support_dynamically_growing_collections() {
        let mut group = AgentCoordinationGroup::new(
            coordination_name("scalable_population"),
            CoordinationStrategy::new(
                coordination_name("collaborative"),
            ),
        );

        // This is a test workload, not an architectural maximum.
        for identity in 1..=256 {
            group.add_participant(
                AgentParticipation::required(
                    agent_id(identity),
                ),
            )
            .expect("participant insertion");
        }

        assert_eq!(group.participants().len(), 256);

        assert!(group.validate().is_ok());
    }

    #[test]
    fn planning_supports_dynamically_growing_dependency_graphs() {
        let mut plan =
            AgentPlan::new(planning_name("scalable_plan"));

        let count = 256usize;

        for index in 0..count {
            let name = format!("step_{index}");

            plan.add_step(planning_step(&name));
        }

        for index in 0..count - 1 {
            let source = format!("step_{index}");
            let target = format!("step_{}", index + 1);

            plan.add_dependency(
                planning_dependency(&source, &target),
            )
            .expect("dependency insertion");
        }

        assert_eq!(plan.step_count(), count);

        let order = plan.topological_order()
            .expect("valid dependency graph");

        assert_eq!(order.len(), count);
    }

    #[test]
    fn agent_contracts_do_not_encode_hardware_limits() {
        let definition = AgentDefinition::new(
            agent_id(u128::MAX),
            qualified_agent_name("PortableAgent"),
        );

        assert_eq!(
            definition.id().get(),
            u128::MAX
        );

        // Hardware feasibility, available memory, device count,
        // placement and execution budgets are not properties
        // of this declarative agent definition.
        //
        // They must be evaluated by the appropriate downstream
        // compiler and execution infrastructure.
    }
}
