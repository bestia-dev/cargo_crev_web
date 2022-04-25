//! automation_tasks_rs for cargo_crev_web

use cargo_auto_lib::*;

/// automation_tasks_rs with_lib
fn main() {
    exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion. Take care to keep them in sync with the changes.

// add or remove tasks that are specific for your project

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("Running automation task: {}", &task);
                if &task == "build" {
                    task_build();
                } else if &task == "build_and_run" {
                    task_build_and_run();
                } else if &task == "release" {
                    task_release();
                } else if &task == "test" {
                    task_test();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
                } else {
                    println!("Task {} is unknown.", &task);
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt
cargo auto build_and_run - builds the crate in debug mode, fmt and runs the server
cargo auto release - builds the crate in release mode, version from date, fmt
cargo auto test - runs all the tests
cargo auto doc - builds the docs, copy to docs directory
cargo auto commit_and_push "message" - commits with message and push with mandatory message
      (If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.)
cargo auto publish_to_web - publish to web, git tag

"#
    );
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "build_and_run", "release","test", "doc", "commit_and_push", "publish_to_web"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["with_lib"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    #[rustfmt::skip]
    let shell_commands = [
        "cargo fmt", 
        "cargo build"];
    run_shell_commands(shell_commands.to_vec());
    println!(
        r#"
After `cargo auto build`, run the tests and the code. If ok, then 
run `cargo auto release`
"#
    );
}

/// cargo build and run
fn task_build_and_run() {
    task_build();
    // TODO: how to stop execution if there is error?
    // copy the bin to web_server_folder
    run_shell_command("cp target/debug/cargo_crev_web ./web_server_folder");
    let cargo_toml = CargoToml::read();
    // open browser with xdg-open
    // for WSL2 in Win10 I used my project https://crates.io/crates/wsl_open_browser
    let x = std::process::Command::new("xdg-open")
        .arg("http://127.0.0.1:8051/rust-reviews/")
        .spawn()
        .expect("Failed to open default browser using `xdg-open`. Probably it is not preinstalled on your Linux distro. Try to instal it with `xdg-utils`.");
    drop(x);
    // start server
    run_shell_command(&format!("cd web_server_folder; ./{}", cargo_toml.package_name()));
    println!(
        r#"
After `cargo auto build_and_run` close the CLI with ctrl+c and close the browser.
"#
    );
}

/// cargo build --release
fn task_release() {
    auto_version_from_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("cargo build --release");
    let cargo_toml = CargoToml::read();
    // uncomment if you want smaller binary files
    run_shell_command(&format!("strip target/release/{}", cargo_toml.package_name()));
    run_shell_command("cp target/release/cargo_crev_web ./web_server_folder");
    println!(
        r#"
After `cargo auto release`, 
run the `cargo auto test`. If ok, then 
run `cargo auto doc`
"#
    );
}

/// cargo test
fn task_test() {
    run_shell_command("cargo test");
   
    println!(
        r#"
After `cargo auto test`, if ok, then 
run `cargo auto doc`
"#
    );
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    auto_md_to_doc_comments();
    let cargo_toml = CargoToml::read();
    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items",
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",cargo_toml.package_name().replace("-","_")) ,
    ];
    run_shell_commands(shell_commands.to_vec());
    // message to help user with next move
    println!(
        r#"
After `cargo auto doc`, check `docs/index.html`. If ok, then 
run `cargo auto commit_and_push "message"` with mandatory commit message
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    match arg_2 {
        None => println!("Error: message for commit is mandatory"),
        Some(message) => {
            run_shell_command(&format!(r#"git add -A && git commit -m "{}""#, message));
            run_shell_command("git push");
            println!(
                r#"
After `cargo auto commit_and_push "message"`
run `cargo auto publish_to_web`
"#
            );
        }
    }
}

/// publish to web and git tag
fn task_publish_to_web() {
    let cargo_toml = CargoToml::read();
    // git tag
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    run_shell_command(&shell_command);

    // copy sh scripts
    // 1. sync files from the Rust project to a local copy of the web folder
    let project_folder_to_publish = format!(r#"~/rustprojects/{package_name}/var_www_scripts/{package_name}/"#, package_name = cargo_toml.package_name());
    let local_copy_of_web_folder = format!(r#"~/rustprojects/googlecloud/var/www/scripts/{package_name}/"#,package_name = cargo_toml.package_name());
    run_shell_command(&format!(r#"rsync -a --info=progress2 --delete-after {} {}"#,project_folder_to_publish, local_copy_of_web_folder));
    // 2. sync files from the local copy to a transfer folder on the remote server
    let ssh_user_and_server = "luciano_bestia@bestia.dev";
    let web_folder_over_ssh = format!(r#"{ssh_user_and_server}:/var/www/scripts/{package_name}/"#,ssh_user_and_server =ssh_user_and_server ,package_name = cargo_toml.package_name());
    run_shell_command(&format!(r#"rsync -e ssh -a --info=progress2 --delete-after {} {}"#,local_copy_of_web_folder, web_folder_over_ssh));
   
    // cargo publish in 3 steps
    // 1. sync files from the Rust project to a local copy of the web folder
    let project_folder_to_publish = format!(r#"~/rustprojects/{package_name}/web_server_folder/"#, package_name = cargo_toml.package_name());
    let local_copy_of_web_folder = format!(r#"~/rustprojects/googlecloud/var/www/webapps/{package_name}/"#,package_name = cargo_toml.package_name());
    run_shell_command(&format!(r#"rsync -a --info=progress2 --delete-after {} {}"#,project_folder_to_publish, local_copy_of_web_folder));
    // 2. sync files from the local copy to a transfer folder on the remote server. 
    let ssh_user_and_server = "luciano_bestia@bestia.dev";
    let web_folder_over_ssh = format!(r#"{ssh_user_and_server}:/var/www/transfer_folder/webapps/{package_name}/"#,ssh_user_and_server =ssh_user_and_server,package_name = cargo_toml.package_name());
    run_shell_command(&format!(r#"rsync -e ssh -a --info=progress2 --delete-after {} {}"#,local_copy_of_web_folder, web_folder_over_ssh));
    // 3. run a publishing script that will stop the server, copy the transferred files and restart the server    
    let ssh_key_file = "/home/luciano/.ssh/luciano_googlecloud";
    // this script will --exclude 'blocklisted_repos.json' when publishing to the web server.
    let script_for_publishing_on_remote = format!(r#"/var/www/scripts/{package_name}/{package_name}_publish.sh"#, package_name = cargo_toml.package_name());
    run_shell_command(&format!(r#"ssh -tt -i {ssh_key_file} {ssh_user_and_server} {script_for_publishing_on_remote}"#,
    ssh_key_file=ssh_key_file,
    ssh_user_and_server=ssh_user_and_server,
    script_for_publishing_on_remote = script_for_publishing_on_remote));

    let web_app_url = format!(r#"https://web.crev.dev/rust-reviews/"#);
    println!(
        r#"
After `cargo auto task_publish_to_web', 
open the browser on {}.
"#,
web_app_url
    );
}

// endregion: tasks

