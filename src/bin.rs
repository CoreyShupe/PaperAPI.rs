extern crate paper_api;
extern crate tokio;
#[macro_use]
extern crate clap;

use paper_api::{PaperClientDebug, PaperClientConfig, PaperClient};
use clap::ArgMatches;
use paper_api::paper::{ChangesInfo, BuildDownloadRequest};
use std::path::Path;
use std::io::Write;

#[tokio::main]
async fn main() -> paper_api::Result<()> {
    let app_matcher = clap_app!(PaperAPI =>
        (@setting SubcommandRequiredElseHelp)
        (version: "0.0.1")
        (author: "Corey Shupe")
        (about: "Command interface to paper's API.")
        (@arg debug: -d --debug "Denotes if there should be debug generated.")
        (@subcommand projects =>
            (about: "Gathers a list of projects supported by paper.")
        )
        (@subcommand download =>
            (about: "Downloads a specific project from the paper API.")
            (@arg path: -P --path +takes_value +required "The path to download to.")
            (@arg project: -p --project +takes_value +required "The project to target.")
            (@arg version: -v --version +takes_value "The project to download. Default: latest")
            (@arg build: -b --build +takes_value "The build to target (number). Default: latest")
        )
        (@subcommand project =>
            (about: "Gathers project information of a specific project.")
            (@arg project: -p --project +takes_value +required "The project to gather information about.")
            (@group vtype =>
                (@arg group: -g --group +takes_value "Defines a version group.")
                (@arg version: -v --version +takes_value "Defines a version.")
            )
            (@subcommand builds =>
                (about: "Gathers the list of builds.")
                (@arg build: -b --build +takes_value "The targeted build.")
            )
        )
    ).get_matches();

    if app_matcher.is_present("debug") {
        handle_matches::<PaperClientDebug>(app_matcher).await?;
    } else {
        handle_matches::<PaperClient>(app_matcher).await?;
    }

    Ok(())
}

async fn handle_matches<ClientConfig>(matcher: ArgMatches<'_>) -> paper_api::Result<()>
    where ClientConfig: PaperClientConfig + Send
{
    match matcher.subcommand_name() {
        Some("projects") => {
            let response = ClientConfig::get_projects().await;
            match response {
                Ok(projects) => {
                    println!("Projects: \t{:?}", projects.projects);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Some("download") => {
            let download_command = matcher.subcommand_matches("download")
                .expect("Sub command must be \"download\".");

            let path = download_command.value_of("path").expect("Arg path required.");
            let project = download_command.value_of("project").expect("Arg project required.");
            let version_str = download_command.value_of("version").unwrap_or("latest");
            let build_str = download_command.value_of("build").unwrap_or("latest");

            let version = if version_str.eq("latest") {
                let project_info = ClientConfig::get_project(project).await?;
                project_info.versions[project_info.versions.len() - 1].to_owned()
            } else {
                String::from(version_str)
            };
            let build = if build_str.eq("latest") {
                let version_info = ClientConfig::get_version_info(project, &version).await?;
                version_info.builds[version_info.builds.len() - 1]
            } else {
                if let Ok(b_i32) = build_str.parse::<i32>() {
                    b_i32
                } else {
                    println!("Build must be defined as a number or \"latest\".");
                    return Ok(());
                }
            };
            let download_info = ClientConfig::get_version_builds(project, &version, build).await?;
            let download = download_info.downloads.application.name;

            let path_buf = Path::new(path);

            let file_path = if path_buf.is_dir() {
                path_buf.join(&Path::new(&*download))
            } else {
                path_buf.to_path_buf()
            };

            let mut file = std::fs::File::create(&file_path)?;
            BuildDownloadRequest::new(project, &version, build, &*download).call::<ClientConfig, _>(move |bytes|
                file.write_all(bytes).expect("Could not write bytes to file.")
            ).await?;

            println!("Downloaded {} to {}", download, file_path.to_str().unwrap());
        }
        Some("project") => {
            let project_command = matcher.subcommand_matches("project")
                .expect("Sub command must be \"project\".");

            let project = project_command.value_of("project").expect("Arg project required.");
            let version = project_command.value_of("version");
            let group = project_command.value_of("group");

            match project_command.subcommand_name() {
                Some("builds") => {
                    let build_command = project_command.subcommand_matches("builds")
                        .expect("Sub command must be \"builds\".");

                    let build = build_command.value_of("build");
                    if let Some(v) = version {
                        if let Some(b) = build {
                            let build_i32 = b.parse::<i32>();
                            if let Ok(b_i32) = build_i32 {
                                let response = ClientConfig::get_version_builds(project, v, b_i32).await;
                                match response {
                                    Ok(info) => {
                                        print_changes(&info.changes);
                                        println!("Project ID:    \t{}", info.project_id);
                                        println!("Project Name:  \t{}", info.project_name);
                                        println!("Version:       \t{}", info.version);
                                        println!("Time:          \t{}", info.time);
                                        println!("{:?}", info.downloads);
                                    }
                                    Err(e) => {
                                        println!("Error: {}", e);
                                    }
                                }
                            } else {
                                println!("Build must be a number.");
                            }
                        } else {
                            let response = ClientConfig::get_version_info(project, v).await;
                            match response {
                                Ok(info) => {
                                    println!("Project ID:   \t{}", info.project_id);
                                    println!("Project Name: \t{}", info.project_name);
                                    println!("Version:      \t{}", info.version);
                                    println!("Builds:       \t{:?}", info.builds);
                                }
                                Err(e) => {
                                    println!("Error: {}", e);
                                }
                            }
                        }
                    } else if let Some(g) = group {
                        let response = ClientConfig::get_group_builds(project, g).await;
                        match response {
                            Ok(info) => {
                                if let Some(b) = build {
                                    let build_i32 = b.parse::<i32>();

                                    if let Ok(b_i32) = build_i32 {
                                        println!("Project ID:    \t{}", info.project_id);
                                        println!("Project Name:  \t{}", info.project_name);
                                        println!("Version Group: \t{}", info.version_group);
                                        println!("Versions:      \t{:?}", info.versions);
                                        let mut flag = false;
                                        for build_info in info.builds {
                                            if build_info.build == b_i32 {
                                                print_changes(&build_info.changes);
                                                println!("Time:          \t{}", build_info.time);
                                                println!("Version:       \t{}", build_info.version);
                                                println!("{:?}", build_info.downloads);
                                                flag = true;
                                                break;
                                            }
                                        }
                                        if !flag {
                                            println!("Build not found in this group.");
                                        }
                                    } else {
                                        println!("Build must be a number.");
                                    }
                                } else {
                                    println!("Project ID:    \t{}", info.project_id);
                                    println!("Project Name:  \t{}", info.project_name);
                                    println!("Version Group: \t{}", info.version_group);
                                    println!("Versions:      \t{:?}", info.versions);
                                    for build_info in info.builds {
                                        println!("\t{}: {} at {}", build_info.version, build_info.build, build_info.time)
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    } else {
                        println!("Group or version required to display build information.")
                    }
                }
                None => {
                    if let Some(v) = version {
                        let response = ClientConfig::get_version_info(project, v).await;
                        match response {
                            Ok(info) => {
                                println!("Project ID:   \t{}", info.project_id);
                                println!("Project Name: \t{}", info.project_name);
                                println!("Version:      \t{}", info.version);
                                println!("Builds:       \t{:?}", info.builds);
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    } else if let Some(g) = group {
                        let response = ClientConfig::get_group_info(project, g).await;
                        match response {
                            Ok(info) => {
                                println!("Project ID:    \t{}", info.project_id);
                                println!("Project Name:  \t{}", info.project_name);
                                println!("Version Group: \t{}", info.version_group);
                                println!("Versions:      \t{:?}", info.versions);
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    } else {
                        let response = ClientConfig::get_project(project).await;
                        match response {
                            Ok(project_info) => {
                                println!("Project ID:             \t{}", project_info.project_id);
                                println!("Project Name:           \t{}", project_info.project_name);
                                println!("Project Version Groups: \t{:?}", project_info.version_groups);
                                println!("Project Versions:       \t{:?}", project_info.versions);
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
                }
                _ => unreachable!()
            }
        }
        _ => unreachable!()
    };
    Ok(())
}

fn print_changes(info: &Vec<ChangesInfo>) {
    println!("Changes:");
    for change_info in info {
        println!("\tCommit: {}", change_info.commit);
        println!("\tMessage: \n`\n{}\n`", change_info.message);
        println!("\tSummary: {}", change_info.summary);
        println!();
    }
}