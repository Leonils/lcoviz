use std::path::PathBuf;

use pathdiff::diff_paths;

use crate::core::{LinkPayload, LinksComputer, WithPath};

pub struct MpaLinksComputer;
impl MpaLinksComputer {
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

    fn get_path_to_root(&self, root: &impl WithPath, file: &impl WithPath) -> PathBuf {
        match file.is_dir() {
            true => root.get_path_relative_to(&file.get_path()),
            false => {
                let file_path = file.get_path();
                let parent_path = file_path.parent().expect(
                    format!(
                        "File '{}', '{}' has no parent",
                        file.get_path_string(),
                        file.get_name()
                    )
                    .as_str(),
                );
                diff_paths(&root.get_path(), &parent_path).unwrap_or_default()
            }
        }
    }
}
impl LinksComputer for MpaLinksComputer {
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

        let file_dir_path = match file.is_dir() {
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
        let target = match file.is_dir() {
            true => Self::get_link_to_dir(root, file),
            false => Self::get_link_to_file(root, file, &file_path),
        };
        LinkPayload {
            link: target.to_str().unwrap().to_string(),
            text: file.get_name().to_string(),
        }
    }

    fn get_link_to_resource(
        &self,
        root: &impl WithPath,
        current: &impl WithPath,
        resource_name: &str,
    ) -> String {
        let target = &self
            .get_path_to_root(root, current)
            .join("_resources")
            .join(resource_name);
        target.to_str().unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        adapters::exporters::mpa_links::MpaLinksComputer,
        core::{LinksComputer, WithPath},
    };

    struct MockWithPath {
        name: String,
        path: PathBuf,
        is_dir: bool,
    }
    impl MockWithPath {
        fn new(name: &str, path: &str, is_dir: bool) -> Self {
            Self {
                name: name.to_string(),
                path: PathBuf::from(path),
                is_dir,
            }
        }
    }
    impl WithPath for MockWithPath {
        fn get_name(&self) -> &str {
            self.name.as_str()
        }
        fn is_dir(&self) -> bool {
            self.is_dir
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
        let root = MockWithPath::new("root", "/root", true);
        let file = MockWithPath::new("file.rs", "/root/dir/file.rs", false);
        let computer = MpaLinksComputer;

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
        let root = MockWithPath::new("root", "/root", true);
        let file = MockWithPath::new("module", "/root/module", true);
        let computer = MpaLinksComputer;

        let links = computer
            .get_links_from_file(&root, &file)
            .collect::<Vec<_>>();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].link, "../index.html");
        assert_eq!(links[0].text, "root");
    }

    #[test]
    fn when_getting_links_to_module_nested_shall_return_links_to_index_files() {
        let root = MockWithPath::new("root", "/root", true);
        let file = MockWithPath::new("module", "/root/dir/module/", true);
        let computer = MpaLinksComputer;

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
        let root = MockWithPath::new("root", "/root", true);
        let file = MockWithPath::new("root", "/root", true);
        let computer = MpaLinksComputer;

        let links = computer
            .get_links_from_file(&root, &file)
            .collect::<Vec<_>>();
        assert_eq!(links.len(), 0);
    }

    #[test]
    fn when_getting_link_to_file_it_shall_add_html_extension() {
        let root = MockWithPath::new("root", "/root", true);
        let file = MockWithPath::new("file.rs", "/root/dir/file.rs", false);
        let computer = MpaLinksComputer;

        let link = computer.get_link_to(&root, &file);
        assert_eq!(link.link, "dir/file.rs.html");
        assert_eq!(link.text, "file.rs");
    }

    #[test]
    fn when_getting_link_to_file_from_empty_root_it_shall_add_html_extension() {
        let root = MockWithPath::new("root", "", true);
        let file = MockWithPath::new("file.rs", "dir/file.rs", false);
        let computer = MpaLinksComputer;

        let link = computer.get_link_to(&root, &file);
        assert_eq!(link.link, "dir/file.rs.html");
        assert_eq!(link.text, "file.rs");
    }

    #[test]
    fn when_getting_link_to_module_it_shall_add_index_html_to_path() {
        let root = MockWithPath::new("root", "/root", true);
        let file = MockWithPath::new("module", "/root/dir/module/", true);
        let computer = MpaLinksComputer;

        let link = computer.get_link_to(&root, &file);
        assert_eq!(link.link, "dir/module/index.html");
        assert_eq!(link.text, "module");
    }

    #[test]
    fn when_getting_resource_from_root_it_shall_get_a_relative_down_path() {
        let root = MockWithPath::new("root", "/root", true);
        let current = MockWithPath::new("root", "/root", true);
        let computer = MpaLinksComputer;

        let link = computer.get_link_to_resource(&root, &current, "resource.svg");
        assert_eq!(link, "_resources/resource.svg");
    }

    #[test]
    fn when_getting_resource_from_module_it_shall_get_a_relative_down_path() {
        let root = MockWithPath::new("root", "/root", true);
        let current = MockWithPath::new("module", "/root/module", true);
        let computer = MpaLinksComputer;

        let link = computer.get_link_to_resource(&root, &current, "resource.svg");
        assert_eq!(link, "../_resources/resource.svg");
    }

    #[test]
    fn when_getting_resource_from_file_it_shall_get_a_relative_down_path() {
        let root = MockWithPath::new("root", "/root", true);
        let current = MockWithPath::new("file.rs", "/root/module/file.rs", false);
        let computer = MpaLinksComputer;

        let link = computer.get_link_to_resource(&root, &current, "resource.svg");
        assert_eq!(link, "../_resources/resource.svg");
    }
}
