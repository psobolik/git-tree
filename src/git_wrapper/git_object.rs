use super::git_object_type::GitObjectType;

/// Models the fields returned by git's ls-tree command
#[derive(Debug)]
pub struct GitObject {
    mode: u32,
    obj_type: Option<GitObjectType>,
    name: String,
    size: Option<usize>,
    path: String,
}

impl GitObject {
    pub fn new(
        mode: u32,
        obj_type: Option<GitObjectType>,
        name: String,
        size: Option<usize>,
        path: String,
    ) -> GitObject {
        GitObject {
            mode,
            obj_type,
            name,
            size,
            path,
        }
    }
    #[allow(dead_code)]
    pub fn mode(&self) -> u32 {
        self.mode
    }
    pub fn obj_type(&self) -> &Option<GitObjectType> {
        &self.obj_type
    }
    #[allow(dead_code)]
    pub fn name(&self) -> &String {
        &self.name
    }
    #[allow(dead_code)]
    pub fn size(&self) -> Option<usize> {
        self.size
    }
    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn is_tree(&self) -> bool {
        if let Some(git_object_type) = self.obj_type() {
            *git_object_type == GitObjectType::Tree
        } else {
            false
        }
    }
}
