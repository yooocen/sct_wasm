struct Param<T> where T:Fn(&str) -> String {
    GetP: T
}

impl Param<T> where T:Fn(&str) -> String {
    pub fn new(fun : T) -> Param<T> {
        Param {
            GetP: fun
        }
    }
    pub fn getP(&self, paramCode: &str) -> String {
        self.GetP(paramCode)
    }
}

