trait Policy<A> {
    fn export_check(self, ctxt); /* how to represent ctxt? */
    fn merge(self, _other Policy<A>);
}

struct Grade {
    studentId: Number, 
    grade: Number, 
};

impl Policy<Grade> for Grade {
    fn export_check(self, ctxt) {
       
    }
    fn merge(self, _other Policy<Number>) {
        
    }
}
