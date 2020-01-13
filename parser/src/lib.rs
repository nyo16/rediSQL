use redisql_lib;

use redisql_lib::redisql_error::RediSQLError;

#[derive(Debug, PartialEq, Clone)]
pub struct CreateDB<'s> {
    name: &'s str,
    path: Option<&'s str>,
    can_exists: bool,
    must_create: bool,
}

impl<'s> CreateDB<'s> {
    pub fn parse(args: Vec<&'s str>) -> Result<Self, RediSQLError> {
        let mut args_iter = args.iter();
        args_iter.next();
        let name = match args_iter.next() {
            Some(name) => name,
            None => unimplemented!(),
        };
        let mut createdb = CreateDB {
            name: name,
            path: None,
            can_exists: false,
            must_create: false,
        };
        while let Some(arg) = args_iter.next() {
            let mut arg_string = String::from(*arg);
            arg_string.make_ascii_uppercase();
            match arg_string.as_str() {
                "PATH" => {
                    let path = match args_iter.next() {
                        Some(path) => path,
                        None => unimplemented!(),
                    };
                    createdb.path = Some(path);
                }
                "CAN_EXIST" => {
                    createdb.can_exists = true;
                }
                "MUST_CREATE" => {
                    createdb.must_create = true;
                }
                _ => {}
            }
        }
        if createdb.can_exists && createdb.must_create {}
        if !createdb.can_exists && !createdb.must_create {
            createdb.can_exists = true;
        }
        Ok(createdb)
    }
}

mod test {

    #[test]
    fn simple_tag() {
        assert!(true);
    }
}