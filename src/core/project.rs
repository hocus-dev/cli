pub fn get_project_docker_prefix(project_name: &str) -> String {
    format!("hocus_{}", project_name)
}

pub fn get_docker_container_name(project_name: &str, service_name: &str) -> String {
    let prefix = get_project_docker_prefix(project_name);
    format!("{}_{}_1", prefix, service_name)
}
