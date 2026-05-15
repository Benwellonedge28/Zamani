
//! Zenith Universal Meta-Compiler (UMC) Standard Library
//!
//! This module aggregates and manages all standard library components for Zenith.

pub mod core;
pub mod collections;
pub mod quantum;
pub mod nano;
pub mod mts;
pub mod sankofa;
pub mod reflection;
pub mod ml;
pub mod net;
pub mod fs;
pub mod sync;
pub mod crypto;
pub mod serialize;
pub mod gui;
pub mod db;
pub mod time;
pub mod numeric;
pub mod web;
pub mod ai_reasoning;
pub mod nlp;
pub mod vision;
pub mod robotics;
pub mod agents;
pub mod meta_ops;
pub mod external_services;
pub mod agi_governance;
pub mod human_agi_interaction;
pub mod reality;
pub mod distributed_ledger;
pub mod iot;
pub mod human_interface_devices;
pub mod chat_architect_agent;
pub mod documentation_system;
pub mod omniversal_simulation;
pub mod on_device_agents;
pub mod resource_management;
pub mod developer_relations;
pub mod omniversal_nlp;
pub mod omniversal_sovereignty;
pub mod omniversal_nlp_adv;
pub mod multidimensional;
pub mod math_foundations;
pub mod network;
pub mod music_language;
pub mod physical_hardware_control;
pub mod mgns;
pub mod test_framework;
pub mod editor_integration;
pub mod system_design; // New: For Autonomous System Design (ASD)

/// Initializes all standard library components.
pub fn initialize_stdlib() {
    println!("Initializing Zenith UMC Standard Library...");
    core::init_core_lib();
    collections::init_collections_lib();
    quantum::init_quantum_lib();
    nano::init_nano_lib();
    mts::init_mts_lib();
    sankofa::init_sankofa_lib();
    reflection::init_reflection_lib();
    ml::init_ml_lib();
    net::init_net_lib();
    fs::init_fs_lib();
    sync::init_sync_lib();
    crypto::init_crypto_lib();
    serialize::init_serialize_lib();
    gui::init_gui_lib();
    db::init_db_lib();
    time::init_time_lib();
    numeric::init_numeric_lib();
    web::init_web_lib();
    ai_reasoning::init_ai_reasoning_lib();
    nlp::init_nlp_lib();
    vision::init_vision_lib();
    robotics::init_robotics_lib();
    agents::init_agents_lib();
    meta_ops::init_meta_ops_lib();
    external_services::init_external_services_lib();
    agi_governance::init_agi_governance_lib();
    human_agi_interaction::init_human_agi_lib();
    reality::init_reality_lib();
    distributed_ledger::init_ledger_lib();
    iot::init_iot_lib();
    human_interface_devices::init_hid_lib();
    chat_architect_agent::init_chat_architect_agent();
    documentation_system::init_documentation_system();
    omniversal_simulation::init_omniversal_simulation();
    on_device_agents::init_on_device_agents();
    resource_management::init_resource_management();
    developer_relations::init_developer_relations();
    omniversal_nlp::init_omniversal_nlp();
    omniversal_sovereignty::init_omniversal_sovereignty();
    omniversal_nlp_adv::init_omniversal_nlp_adv();
    multidimensional::init_multidimensional();
    math_foundations::init_math_foundations();
    network::init_network_stack();
    music_language::init_music_language();
    physical_hardware_control::init_physical_hardware_control();
    mgns::init_mgns();
    test_framework::init_test_framework();
    editor_integration::init_editor_integration();
    system_design::init_system_design(); // Initialize ASD module
    println!("Zenith UMC Standard Library initialized.");
}

/// Shuts down all standard library components.
pub fn shutdown_stdlib() {
    println!("Shutting down Zenith UMC Standard Library...");
    system_design::shutdown_system_design(); // Shutdown ASD module
    editor_integration::shutdown_editor_integration();
    test_framework::shutdown_test_framework();
    mgns::shutdown_mgns();
    physical_hardware_control::shutdown_physical_hardware_control();
    music_language::shutdown_music_language();
    network::shutdown_network_stack();
    math_foundations::shutdown_math_foundations();
    multidimensional::shutdown_multidimensional();
    omniversal_nlp_adv::shutdown_omniversal_nlp_adv();
    omniversal_sovereignty::shutdown_omniversal_sovereignty();
    omniversal_nlp::shutdown_omniversal_nlp();
    developer_relations::shutdown_developer_relations();
    resource_management::shutdown_resource_management();
    on_device_agents::shutdown_on_device_agents();
    omniversal_simulation::shutdown_omniversal_simulation();
    documentation_system::shutdown_documentation_system();
    chat_architect_agent::shutdown_chat_architect_agent();
    human_interface_devices::shutdown_hid_lib();
    iot::shutdown_iot_lib();
    distributed_ledger::shutdown_ledger_lib();
    reality::shutdown_reality_lib();
    human_agi_interaction::shutdown_human_agi_lib(); 
    agi_governance::shutdown_agi_governance_lib();
    external_services::shutdown_external_services_lib();
    meta_ops::shutdown_meta_ops_lib(); 
    agents::shutdown_agents_lib(); 
    robotics::shutdown_robotics_lib(); 
    vision::shutdown_vision_lib(); 
    nlp::shutdown_nlp_lib(); 
    ai_reasoning::shutdown_ai_reasoning_lib(); 
    web::shutdown_web_lib(); 
    numeric::shutdown_numeric_lib(); 
    time::shutdown_time_lib(); 
    db::shutdown_db_lib(); 
    gui::shutdown_gui_lib(); 
    serialize::init_serialize_lib(); 
    crypto::shutdown_crypto_lib(); 
    sync::shutdown_sync_lib(); 
    fs::shutdown_fs_lib(); 
    net::shutdown_net_lib(); 
    ml::shutdown_ml_lib(); 
    reflection::shutdown_reflection_lib(); 
    sankofa::shutdown_sankofa_lib();
    mts::shutdown_mts_lib();
    nano::shutdown_nano_lib();
    quantum::shutdown_quantum_lib();
    collections::shutdown_collections_lib();
    core::shutdown_core_lib();
    println!("Zenith UMC Standard Library shut down.");
}
