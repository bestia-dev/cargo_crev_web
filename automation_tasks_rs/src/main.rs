// automation_tasks_rs for cargo_crev_web

// region: library with basic automation tasks
use cargo_auto_lib as cl;
// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

use cargo_auto_lib::GREEN;
use cargo_auto_lib::RED;
use cargo_auto_lib::RESET;
use cargo_auto_lib::YELLOW;

// region: library with basic automation tasks

fn main() {
    cl::exit_if_not_run_in_rust_project_root_directory();

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

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
                println!("{YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_web" {
                    task_publish_to_web();
                } else {
                    println!("{RED}Error: Task {task} is unknown.{RESET}");
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
    {YELLOW}Welcome to cargo-auto !{RESET}
    {YELLOW}This program automates your custom tasks when developing a Rust project.{RESET}

    {YELLOW}User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto build{RESET} - {YELLOW}builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto release{RESET} - {YELLOW}builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto doc{RESET} - {YELLOW}builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET} - {YELLOW}runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET} - {YELLOW}commits with message and push with mandatory message{RESET}
    {YELLOW}(If you use SSH, it is easy to start the ssh-agent in the background and ssh-add your credentials for git.){RESET}
{GREEN}cargo auto publish_to_web - publish to web, git tag{RESET}

    {YELLOW}© 2024 bestia.dev  MIT License github.com/bestia-dev/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd(){
/*
    println!(r#"{YELLOW}run examples:{RESET}
{GREEN}cargo run --example example1{RESET}
"#);
*/
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "test", "commit_and_push", "publish_to_web"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("cargo build");
    cl::run_shell_command("cp target/debug/cargo_crev_web ./web_server_folder");
    println!(
        r#"
    {YELLOW}After `cargo auto build`, run the compiled binary, examples and/or tests{RESET}
    {YELLOW}Run the web server in a separate terminal, so you can use the first terminal{RESET}
{GREEN}cd ~/rustprojects/cargo_crev_web/web_server_folder; ./{package_name}; {RESET}
    {YELLOW}Forward the port 8051 in VSCode and then open in browser{RESET}
{GREEN}http://127.0.0.1:8051/rust-reviews/{RESET}    
    {YELLOW}if ok then{RESET}
{GREEN}cargo auto release{RESET}
"#,
package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo build --release
fn task_release() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command("cargo fmt");
    cl::run_shell_command("cargo build --release");
    cl::run_shell_command(&format!(
        "strip target/release/{package_name}",
        package_name = cargo_toml.package_name()
    )); 
    cl::run_shell_command("cp target/release/cargo_crev_web ./web_server_folder");
    println!(
        r#"
    {YELLOW}After `cargo auto release`, run the compiled binary, examples and/or tests{RESET}
    {YELLOW}Run the web server in a separate terminal, so you can use the first terminal{RESET}
{GREEN}cd ~/rustprojects/cargo_crev_web/web_server_folder; ./{package_name}; {RESET}
    {YELLOW}Forward the port 8051 in VSCode and then open in browser{RESET}
{GREEN}http://127.0.0.1:8051/rust-reviews/{RESET}    
    {YELLOW}if ok then{RESET}
{GREEN}cargo auto doc{RESET}
"#,
package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a github standard folder
fn task_doc() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");
    cl::auto_plantuml(&cargo_toml.package_repository().unwrap());
    cl::auto_playground_run_code();
    cl::auto_md_to_doc_comments();

    cl::run_shell_command("cargo doc --no-deps --document-private-items");
    // copy target/doc into docs/ because it is github standard
    cl::run_shell_command("rsync -a --info=progress2 --delete-after target/doc/ docs/");
    // Create simple index.html file in docs directory
    cl::run_shell_command(&format!(
        r#"echo "<meta http-equiv=\"refresh\" content=\"0; url={}/index.html\" />" > docs/index.html"#,
        cargo_toml.package_name().replace("-", "_")
    ));
    // pretty html
    cl::auto_doc_tidy_html().unwrap();
    cl::run_shell_command("cargo fmt");
    // message to help user with next move
    println!(
        r#"
    {YELLOW}After `cargo auto doc`, check `docs/index.html`. If ok then test the documentation code examples{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
}

/// cargo test
fn task_test() {
    cl::run_shell_command("cargo test");
    println!(
r#"
    {YELLOW}After `cargo auto test`. If ok then {RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
    {YELLOW}with mandatory commit message{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    let Some(message) = arg_2 else {
        eprintln!("{RED}Error: Message for commit is mandatory. Exiting.{RESET}");
        // early exit
        return;
    };

    // init repository if needed. If it is not init then normal commit and push.
    if !cl::init_repository_if_needed(&message) {
        // separate commit for docs if they changed, to not make a lot of noise in the real commit
        if std::path::Path::new("docs").exists() {
            cl::run_shell_command(r#"git add docs && git diff --staged --quiet || git commit -m "update docs" "#);
        }
        cl::add_message_to_unreleased(&message);
        // the real commit of code
        cl::run_shell_command(&format!( r#"git add -A && git diff --staged --quiet || git commit -m "{message}" "#));
        cl::run_shell_command("git push");
        println!(
r#"
    {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
        );
    }
}

/// publish to web and git tag
fn task_publish_to_web() {
    let cargo_toml = cl::CargoToml::read();
    let package_name = cargo_toml.package_name();
    // git tag
    println!(r#"    {YELLOW}git tag {RESET}"#);
    let shell_command = format!(
        "git tag -f -a v{version} -m version_{version}",
        version = cargo_toml.package_version()
    );
    cl::run_shell_command(&shell_command);

    println!(r#"    {YELLOW}1. upload sh scripts {RESET}"#);
    let project_folder_to_publish = format!(r#"~/rustprojects/{package_name}/var_www_scripts/{package_name}/"#);
    let ssh_user_and_server = "luciano_bestia@bestia.dev";
    let ssh_key_file = "/home/rustdevuser/.ssh/bestia_dev_ssh_1";
    let remote_temp_folder = format!(r#"/tmp/bestia-dev/{package_name}/"# );
    let web_folder_over_ssh = format!(r#"{ssh_user_and_server}:{remote_temp_folder}"# );
    cl::run_shell_command(&format!(r#"ssh -i {ssh_key_file} {ssh_user_and_server} mkdir -p {remote_temp_folder}"#));
    cl::run_shell_command(&format!(r#"rsync -e ssh -a --info=progress2 --delete-after {project_folder_to_publish} {web_folder_over_ssh}"#));
    cl::run_shell_command(&format!(r#"ssh -i {ssh_key_file} {ssh_user_and_server} chmod a+rx {remote_temp_folder}/cargo_crev_web_publish.sh"#)); 

    // 2. rsync files from the Rust project to uploaded_web_server_folder on the remote server.
    println!(r#"    {YELLOW}2. upload files to uploaded_web_server_folder on the remote {RESET}"#);
    let project_folder_to_publish = format!(r#"~/rustprojects/{package_name}/web_server_folder/"#);    
    let remote_temp_folder = format!(r#"/tmp/bestia-dev/{package_name}/uploaded_web_server_folder/"# );
    let web_folder_over_ssh = format!(r#"{ssh_user_and_server}:{remote_temp_folder}"#);
    cl::run_shell_command(&format!(r#"ssh -i {ssh_key_file} {ssh_user_and_server} mkdir -p {remote_temp_folder}"#));
    cl::run_shell_command(&format!(r#"rsync -e ssh -a --info=progress2 --delete-after {project_folder_to_publish} {web_folder_over_ssh}"#));
    
    // 3. run the publishing script that will stop the server, copy the transferred files and restart the server    
    // this script will --exclude 'blocklisted_repos.json' when publishing to the web server.
    println!(r#"    {YELLOW}3. run the publishing script on the remote{RESET}"#);
    let script_for_publishing_on_remote = format!(r#"/tmp/bestia-dev/{package_name}/{package_name}_publish.sh"#);
    cl::run_shell_command(&format!(r#"ssh -tt -i {ssh_key_file} {ssh_user_and_server} {script_for_publishing_on_remote}"#));
 
    let web_app_url = format!(r#"https://web.crev.dev/rust-reviews/"#);
    println!(
        r#"
    {YELLOW}After `cargo auto publish_to_web', open the browser on {RESET}
{GREEN}{web_app_url}{RESET}
    {YELLOW}Check over ssh if everything is working right{RESET}
{GREEN}ssh -i {ssh_key_file} {ssh_user_and_server} {RESET}
"#);
}
// endregion: tasks

