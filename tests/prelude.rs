use wisp::builtins::bootstrap_env;
use wisp::eval::eval;
use wisp::reader::read;
use wisp::value::Value;

#[cfg(test)]
mod tests {
    #[test]
    pub fn it_works() {
        assert!(1 + 1 == 2)
    }
}
