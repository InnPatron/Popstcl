#[macro_use]
use std::collections::HashSet;

#[macro_export]
macro_rules! has_permission {
    ($entry: expr, $permission: expr) => {{
        if $entry.permissions().has_permission($permission) == false {
            return Err(ExecErr::NoPermission($permission))
        }
    }};
}

#[macro_export]
macro_rules! observable_internal {
	( ) => {
		{
			EntryPermissions::new()
								.internal_read()
								.internal_write()
								.external_read()
								.foreign_mod_read()
		}
	}
}

#[macro_export]
macro_rules! all_readonly {
	( ) => {
		{
			EntryPermissions::new()
								.internal_read()
								.external_read()
								.foreign_mod_read()
		}
	}
}

#[macro_export]
macro_rules! all_read_write {
	( ) => {
		{
			EntryPermissions::new()
								.internal_read()
								.internal_write()
								.external_read()
                                .external_write()
								.foreign_mod_read()
                                .foreign_mod_write()
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct EntryPermissions(HashSet<Permissions>);

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Permissions {
    InternalRead,       //Can be read from within module
    InternalWrite,      //Can be written from within module

    ForeignModRead,     //Can be read by another module
    ForeignModWrite,    //Can be written by another module
    
    ExternalRead,         //Can be read by non-popstcl code
    ExternalWrite,        //Can be written by non-popstcl code
}

macro_rules! add_permission {
    ($s: ident, $p: expr) => {
        {
            $s.0.insert($p);
            $s
        }
    }
}

impl EntryPermissions {
    pub fn new() -> EntryPermissions {
        EntryPermissions(HashSet::new())
    }

    pub fn from_vec(vec: Vec<Permissions>) -> EntryPermissions {
        let mut entry_p = EntryPermissions::new();
        for p in vec.into_iter() {
            entry_p.add_permission(p);
        }
        entry_p
    }

    pub fn internal_read(mut self) -> EntryPermissions {
        add_permission!(self, Permissions::InternalRead)
    }

    pub fn internal_write(mut self) -> EntryPermissions {
        add_permission!(self, Permissions::InternalWrite)
    }

    pub fn external_read(mut self) -> EntryPermissions {
        add_permission!(self, Permissions::ExternalRead)
    }

    pub fn external_write(mut self) -> EntryPermissions {
        add_permission!(self, Permissions::ExternalWrite)
    }

    pub fn foreign_mod_read(mut self) -> EntryPermissions {
        add_permission!(self, Permissions::ForeignModRead)
    }

    pub fn foreign_mod_write(mut self) -> EntryPermissions {
        add_permission!(self, Permissions::ForeignModWrite)
    }

    pub fn has_permission(&self, permission: Permissions) -> bool {
        self.0.contains(&permission)
    }

    pub fn add_permission(&mut self, permission: Permissions) {
        self.0.insert(permission);
    }

    pub fn remove_permission(&mut self, permission: Permissions) {
        self.0.remove(&permission);
    }
}
