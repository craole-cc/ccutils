use super::name::Name;

pub struct Bash {
    name: Name,
    path: Option<PathBuf>,
    config: Vec<PathBuf>,
}

// impl Default for Bash {
//     fn default() -> Self {
//         Self {
//             name: "bash".to_string(),
//             config: vec![
//                 PathBuf::from(home_dir().unwrap().join(".basrc")),
//                 PathBuf::from(home_dir().unwrap().join(".bash_profile")),
//                 PathBuf::from(home_dir().unwrap().join(".bash_login")),
//                 PathBuf::from("/etc/bash.bashrc"),
//                 PathBuf::from("/etc/bashrc"),
//                 PathBuf::from("/etc/profile"),
//                 PathBuf::from("/etc/profile.d"),
//             ],
//         }
//     }
// }
