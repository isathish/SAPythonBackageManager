use std::path::Path;
use std::fs;
use bollard::Docker;
use futures_util::TryStreamExt;
use tempfile::TempDir;
use colored::*;

// Docker integration
pub struct DockerManager {
    pub docker: Docker,
}

impl DockerManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let docker = Docker::connect_with_local_defaults()?;
        Ok(DockerManager { docker })
    }

    pub async fn create_environment(
        &self,
        name: &str,
        base_image: &str,
        requirements: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", format!("🐳 Creating Docker environment '{}'...", name).cyan());

        // Create Dockerfile content
        let mut dockerfile_content = format!(
            "FROM {}\n\
             WORKDIR /app\n\
             RUN pip install --upgrade pip\n",
            base_image
        );

        if let Some(req_file) = requirements {
            if Path::new(req_file).exists() {
                dockerfile_content.push_str(&format!(
                    "COPY {} /app/requirements.txt\n\
                     RUN pip install -r requirements.txt\n",
                    req_file
                ));
            }
        }

        dockerfile_content.push_str("CMD [\"python\"]\n");

        // Create temporary directory for build context
        let temp_dir = TempDir::new()?;
        let dockerfile_path = temp_dir.path().join("Dockerfile");
        fs::write(&dockerfile_path, dockerfile_content)?;

        if let Some(req_file) = requirements {
            if Path::new(req_file).exists() {
                let dest_path = temp_dir.path().join("requirements.txt");
                fs::copy(req_file, dest_path)?;
            }
        }

        // Build image
        use bollard::image::BuildImageOptions;

        let options = BuildImageOptions {
            dockerfile: "Dockerfile",
            t: name,
            rm: true,
            ..Default::default()
        };

        let mut tar_builder = tar::Builder::new(Vec::new());
        tar_builder.append_dir_all(".", temp_dir.path())?;
        let tar_data = tar_builder.into_inner()?;

        let mut stream = self.docker.build_image(options, None, Some(tar_data.into()));

        while let Some(msg) = stream.try_next().await? {
            if let Some(stream) = msg.stream {
                print!("{}", stream);
            }
        }

        println!("{}", format!("✅ Environment '{}' created successfully", name).green());
        Ok(())
    }

    pub async fn list_environments(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        use bollard::image::ListImagesOptions;

        let options = ListImagesOptions::<String> {
            all: false,
            ..Default::default()
        };

        let images = self.docker.list_images(Some(options)).await?;
        let mut environments = Vec::new();

        for image in images {
            for tag in image.repo_tags {
                if !tag.contains(':') || tag.ends_with(":latest") {
                    environments.push(tag.replace(":latest", ""));
                }
            }
        }

        Ok(environments)
    }

    pub async fn execute_in_environment(
        &self,
        name: &str,
        command: &[String],
    ) -> Result<(), Box<dyn std::error::Error>> {
        use bollard::container::{CreateContainerOptions, Config, StartContainerOptions};

        let container_name = format!("sa-exec-{}", uuid::Uuid::new_v4());

        let config = Config {
            image: Some(name),
            cmd: Some(command.iter().map(|s| s.as_str()).collect()),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            ..Default::default()
        };

        let options = CreateContainerOptions {
            name: container_name.as_str(),
            ..Default::default()
        };

        self.docker.create_container(Some(options), config).await?;

        self.docker.start_container(&container_name, None::<StartContainerOptions<String>>).await?;

        // Wait for container to finish and get logs
        use bollard::container::LogsOptions;

        let logs_options = LogsOptions::<String> {
            stdout: true,
            stderr: true,
            follow: true,
            ..Default::default()
        };

        let mut logs_stream = self.docker.logs(&container_name, Some(logs_options));

        while let Some(log) = logs_stream.try_next().await? {
            print!("{}", log);
        }

        // Clean up container
        use bollard::container::RemoveContainerOptions;
        let remove_options = RemoveContainerOptions {
            force: true,
            ..Default::default()
        };

        self.docker.remove_container(&container_name, Some(remove_options)).await?;

        Ok(())
    }
}
