extern crate paper_api;
extern crate tokio;
#[macro_use]
extern crate clap;

use paper_api::paper::{ProjectRequest, ProjectVersionInfoRequest, ProjectGroupInfoRequest, ProjectGroupBuildsRequest, ProjectVersionBuildsRequest};

#[tokio::main]
async fn main() {
    let app_matcher = clap_app!(PaperAPI =>
        (@setting SubcommandRequiredElseHelp)
        (version: "0.0.1")
        (author: "Corey Shupe")
        (about: "Command interface to paper's API.")
        (@subcommand projects =>
            (about: "Gathers a list of projects supported by paper.")
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

    match app_matcher.subcommand_name() {
        Some("projects") => {
            let response = paper_api::get_projects().await;
            match response {
                Ok(projects) => {
                    println!("Projects: \t{:?}", projects.projects);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Some("project") => {
            let project_command = app_matcher.subcommand_matches("project")
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
                                let response = paper_api::get_version_builds(ProjectVersionBuildsRequest::new(project, v, b_i32)).await;
                                match response {
                                    Ok(info) => {
                                        println!("Changes:");
                                        for change_info in info.changes {
                                            println!("\tCommit: {}", change_info.commit);
                                            println!("\tMessage: \n`\n{}\n`", change_info.message);
                                            println!("\tSummary: {}", change_info.summary);
                                            println!();
                                        }
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
                            let response = paper_api::get_version_info(ProjectVersionInfoRequest::new(project, v)).await;
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
                        let response = paper_api::get_group_builds(ProjectGroupBuildsRequest::new(project, g)).await;
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
                                                println!("Changes:");
                                                for change_info in build_info.changes {
                                                    println!("\tCommit: {}", change_info.commit);
                                                    println!("\tMessage: \n`\n{}\n`", change_info.message);
                                                    println!("\tSummary: {}", change_info.summary);
                                                    println!();
                                                }
                                                println!("Time:          \t{}", build_info.time);
                                                println!("Version:       \t{}", build_info.version);
                                                println!("{:?}", build_info.downloads);
                                                flag = true;
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
                        let response = paper_api::get_version_info(ProjectVersionInfoRequest::new(project, v)).await;
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
                        let response = paper_api::get_group_info(ProjectGroupInfoRequest::new(project, g)).await;
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
                        let response = paper_api::get_project(ProjectRequest::new(project)).await;
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
}