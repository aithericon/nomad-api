#[cfg(test)]
mod tests {
    use nomad_api::client::NomadClient;
    use std::collections::HashMap;

    #[tokio::test]
    async fn list_jobs() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let list_nodes_response = client.list_jobs().await;
        println!("{:?}", &list_nodes_response);
        list_nodes_response.unwrap();
    }

    #[tokio::test]
    async fn list_job_allocations() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let list_job_allocations_response = client.list_job_allocations("numpire-worker-gpu/dispatch-1626361554-dc6a23ac").await;
        println!("{:?}", &list_job_allocations_response);
        list_job_allocations_response.unwrap();
    }

    #[tokio::test]
    async fn read_allocation() {
        let client = NomadClient::new("http://127.0.0.1:7777".to_string(), "".to_string());
        let allocation = client
            .read_allocation("bfc47b74-8cd4-2b16-ef93-c3bed24f54bf")
            .await;
        println!("{:?}", &allocation);
        allocation.unwrap();
    }

    #[tokio::test]
    async fn dispatch_job() {
        let client = NomadClient::new("http://127.0.0.1:4646".to_string(), "".to_string());
        let payload = None;
        let mut meta = HashMap::new();
        meta.insert(
            "numpire_image".to_string(),
            "localhost:5000/agridos-worker".to_string(),
        );
        meta.insert("numpire_node_id".to_string(), "1".to_string());
        let dispatch_result = client
            .dispatch_job("numpire-worker-gpu", payload, meta)
            .await;
        println!("{:?}", &dispatch_result);
        dispatch_result.unwrap();
    }
}
