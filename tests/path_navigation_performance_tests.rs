use model_manager::{
    CoreValue, DefaultModelManager, DefaultValueFactory, ModelManager, ModelManagerFactory,
    PathNavigation, ValueFactory,
};

#[test]
fn test_optimized_path_navigation_performance() {
    let mut data = DefaultValueFactory::create_object();

    for level1 in 0..100 {
        let mut level1_obj = DefaultValueFactory::create_object();
        for level2 in 0..50 {
            let mut level2_obj = DefaultValueFactory::create_object();
            level2_obj
                .set(
                    "value",
                    DefaultValueFactory::create_string(&format!("data_{}_{}", level1, level2)),
                )
                .unwrap();
            level1_obj
                .set(&format!("sub_{}", level2), level2_obj)
                .unwrap();
        }
        data.set(&format!("section_{}", level1), level1_obj)
            .unwrap();
    }

    let start = std::time::Instant::now();

    for i in 0..1000 {
        let level1_idx = i % 100;
        let level2_idx = i % 50;
        let path = format!("section_{}.sub_{}.value", level1_idx, level2_idx);

        let result = data.get_by_path(&path).unwrap();
        assert!(result.is_some());

        let expected = format!("data_{}_{}", level1_idx, level2_idx);
        assert_eq!(result.unwrap().as_str().unwrap(), expected);
    }

    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 100,
        "Path navigation too slow: {:?}",
        duration
    );
    println!("✅ 1000 path navigations completed in {:?}", duration);
}

#[test]
fn test_optimized_deep_path_setting() {
    let mut config = DefaultValueFactory::create_object();

    let paths_and_values = vec![
        ("app.database.primary.host", "db-primary.company.com"),
        ("app.database.primary.port", "5432"),
        ("app.database.replica.host", "db-replica.company.com"),
        ("app.database.replica.port", "5433"),
        ("app.cache.redis.cluster.node1.host", "redis-1.company.com"),
        ("app.cache.redis.cluster.node1.port", "6379"),
        ("app.cache.redis.cluster.node2.host", "redis-2.company.com"),
        ("app.cache.redis.cluster.node2.port", "6380"),
        ("app.logging.level", "INFO"),
        ("app.logging.file.path", "/var/log/app.log"),
        ("app.security.jwt.secret", "super_secret_key"),
        ("app.security.jwt.expiration", "3600"),
    ];

    let start = std::time::Instant::now();

    for (path, value_str) in &paths_and_values {
        config
            .set_by_path(path, DefaultValueFactory::create_string(value_str))
            .unwrap();
    }

    let setting_duration = start.elapsed();

    let start = std::time::Instant::now();

    for (path, expected_value) in &paths_and_values {
        let retrieved = config.get_by_path(path).unwrap().unwrap();
        assert_eq!(retrieved.as_str().unwrap(), *expected_value);
    }

    let retrieval_duration = start.elapsed();

    assert!(setting_duration.as_millis() < 10);
    assert!(retrieval_duration.as_millis() < 5);

    println!(
        "✅ Path setting: {:?}, retrieval: {:?}",
        setting_duration, retrieval_duration
    );
}

#[test]
fn test_model_manager_batch_path_operations() {
    let mut manager = DefaultModelManager::create();

    let mut company_config = DefaultValueFactory::create_object();
    company_config
        .set_by_path(
            "general.name",
            DefaultValueFactory::create_string("TechCorp"),
        )
        .unwrap();
    company_config
        .set_by_path(
            "general.ruc",
            DefaultValueFactory::create_string("20123456789"),
        )
        .unwrap();
    company_config
        .set_by_path(
            "database.host",
            DefaultValueFactory::create_string("localhost"),
        )
        .unwrap();
    company_config
        .set_by_path(
            "database.port",
            DefaultValueFactory::create_number(5432.0).unwrap(),
        )
        .unwrap();
    company_config
        .set_by_path("features.analytics", DefaultValueFactory::create_bool(true))
        .unwrap();

    manager
        .insert(
            "company_config".to_string(),
            Some("main_config".to_string()),
            company_config,
        )
        .unwrap();

    let start = std::time::Instant::now();

    let company_name = manager
        .get_by_path(
            "company_config".to_string(),
            "main_config".to_string(),
            "general.name".to_string(),
        )
        .unwrap()
        .unwrap();

    let db_port = manager
        .get_by_path(
            "company_config".to_string(),
            "main_config".to_string(),
            "database.port".to_string(),
        )
        .unwrap()
        .unwrap();

    let analytics_enabled = manager
        .get_by_path(
            "company_config".to_string(),
            "main_config".to_string(),
            "features.analytics".to_string(),
        )
        .unwrap()
        .unwrap();

    let duration = start.elapsed();

    assert_eq!(company_name.as_str().unwrap(), "TechCorp");
    assert_eq!(db_port.as_number().unwrap(), 5432.0);
    assert_eq!(analytics_enabled.as_bool().unwrap(), true);

    assert!(duration.as_millis() < 5);
    println!(
        "✅ Model manager path operations completed in {:?}",
        duration
    );
}
