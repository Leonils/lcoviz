use std::path::PathBuf;

use pathdiff::diff_paths;

use crate::core::{FileSystem, LinkPayload, LinksComputer, WithPath};

pub struct MpaLinksComputer<'a, TFileSystem: FileSystem> {
    file_system: &'a TFileSystem,
}
impl<'a, TFileSystem: FileSystem> MpaLinksComputer<'a, TFileSystem> {
    pub fn new(file_system: &'a TFileSystem) -> Self {
        Self { file_system }
    }

    fn get_link_to_file(
        root: &impl WithPath,
        file: &impl WithPath,
        file_path: &PathBuf,
    ) -> PathBuf {
        let file_extension = file_path.extension().unwrap_or_default();
        PathBuf::new()
            .join(file.get_path_relative_to(&root.get_path()))
            .with_extension(format!("{}.html", file_extension.to_string_lossy()))
    }

    fn get_link_to_dir(root: &impl WithPath, file: &impl WithPath) -> PathBuf {
        file.get_path_relative_to(&root.get_path())
            .join("index.html")
    }
}
impl<'a, TFileSystem: FileSystem> LinksComputer for MpaLinksComputer<'a, TFileSystem> {
    fn get_links_from_file(
        &self,
        root: &impl WithPath,
        file: &impl WithPath,
    ) -> impl Iterator<Item = LinkPayload> {
        if root.get_path() == file.get_path() {
            return vec![].into_iter();
        }

        let root_path = root.get_path();
        let file_path = file.get_path();

        let file_dir_path = match self.file_system.is_dir(&file_path) {
            true => file_path.to_path_buf(),
            false => file_path.parent().unwrap().to_path_buf(),
        };

        let mut links: Vec<LinkPayload> = Vec::new();
        let root_link = LinkPayload {
            link: root
                .get_path_relative_to(&file_dir_path)
                .join("index.html")
                .to_str()
                .unwrap()
                .to_string(),
            text: root.get_name().to_string(),
        };

        for ancestor in file_path.ancestors().skip(1) {
            if ancestor == root_path {
                break;
            }

            let target = diff_paths(ancestor, &file_dir_path)
                .unwrap()
                .join("index.html");
            let link = LinkPayload {
                link: target.to_str().unwrap().to_string(),
                text: ancestor
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap()
                    .to_string(),
            };

            links.push(link);
        }

        links.push(root_link);
        links.reverse();
        links.into_iter()
    }

    fn get_link_to(&self, root: &impl WithPath, file: &impl WithPath) -> LinkPayload {
        let file_path = file.get_path();
        let target = match self.file_system.is_dir(&file_path) {
            true => Self::get_link_to_dir(root, file),
            false => Self::get_link_to_file(root, file, &file_path),
        };
        LinkPayload {
            link: target.to_str().unwrap().to_string(),
            text: file.get_name().to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::core::{LinksComputer, MockFileSystem, WithPath};

    struct MockWithPath {
        name: String,
        path: PathBuf,
    }
    impl MockWithPath {
        fn new(name: &str, path: &str) -> Self {
            Self {
                name: name.to_string(),
                path: PathBuf::from(path),
            }
        }
    }
    impl WithPath for MockWithPath {
        fn get_name(&self) -> &str {
            self.name.as_str()
        }
        fn get_path(&self) -> std::path::PathBuf {
            self.path.clone()
        }
        fn get_path_string(&self) -> String {
            self.path.to_str().unwrap().to_string()
        }
    }

    #[test]
    fn when_getting_links_to_parent_modules_from_file_shall_return_links_to_index_files() {
        let root = MockWithPath::new("root", "/root");
        let file = MockWithPath::new("file.rs", "/root/dir/file.rs");

        let mut fs = MockFileSystem::new();
        fs.expect_is_dir().times(1).return_const(false);
        let computer = super::MpaLinksComputer { file_system: &fs };

        let links = computer
            .get_links_from_file(&root, &file)
            .collect::<Vec<_>>();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].link, "../index.html");
        assert_eq!(links[0].text, "root");
        assert_eq!(links[1].link, "index.html");
        assert_eq!(links[1].text, "dir");
    }

    #[test]
    fn when_getting_links_to_parent_modules_from_dir_shall_return_links_to_index_files() {
        let root = MockWithPath::new("root", "/root");
        let file = MockWithPath::new("module", "/root/module");

        let mut fs = MockFileSystem::new();
        fs.expect_is_dir().times(1).return_const(true);
        let computer = super::MpaLinksComputer { file_system: &fs };

        let links = computer
            .get_links_from_file(&root, &file)
            .collect::<Vec<_>>();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].link, "../index.html");
        assert_eq!(links[0].text, "root");
    }

    #[test]
    fn when_getting_links_to_module_nested_shall_return_links_to_index_files() {
        let root = MockWithPath::new("root", "/root");
        let file = MockWithPath::new("module", "/root/dir/module/");
        let mut fs = MockFileSystem::new();
        fs.expect_is_dir().times(1).return_const(true);
        let computer = super::MpaLinksComputer { file_system: &fs };

        let links = computer
            .get_links_from_file(&root, &file)
            .collect::<Vec<_>>();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].link, "../../index.html");
        assert_eq!(links[0].text, "root");
        assert_eq!(links[1].link, "../index.html");
        assert_eq!(links[1].text, "dir");
    }

    #[test]
    fn when_getting_links_to_module_from_root_shall_return_empty() {
        let root = MockWithPath::new("root", "/root");
        let file = MockWithPath::new("root", "/root");
        let fs = MockFileSystem::new();
        let computer = super::MpaLinksComputer { file_system: &fs };

        let links = computer
            .get_links_from_file(&root, &file)
            .collect::<Vec<_>>();
        assert_eq!(links.len(), 0);
    }

    #[test]
    fn when_getting_link_to_file_it_shall_add_html_extension() {
        let root = MockWithPath::new("root", "/root");
        let file = MockWithPath::new("file.rs", "/root/dir/file.rs");
        let mut fs = MockFileSystem::new();
        fs.expect_is_dir().times(1).return_const(false);
        let computer = super::MpaLinksComputer { file_system: &fs };

        let link = computer.get_link_to(&root, &file);
        assert_eq!(link.link, "dir/file.rs.html");
        assert_eq!(link.text, "file.rs");
    }

    #[test]
    fn when_getting_link_to_module_it_shall_add_index_html_to_path() {
        let root = MockWithPath::new("root", "/root");
        let file = MockWithPath::new("module", "/root/dir/module/");
        let mut fs = MockFileSystem::new();
        fs.expect_is_dir().times(1).return_const(true);
        let computer = super::MpaLinksComputer { file_system: &fs };

        let link = computer.get_link_to(&root, &file);
        assert_eq!(link.link, "dir/module/index.html");
        assert_eq!(link.text, "module");
    }
}
