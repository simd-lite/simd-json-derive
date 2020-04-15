impl _serde::Serialize for Bla {
    fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
    where
        __S: _serde::Serializer,
    {
        let mut __serde_state =
            match _serde::Serializer::serialize_struct(__serializer, "Bla", false as usize + 1 + 1)
            {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "f1", &self.f1) {
            _serde::export::Ok(__val) => __val,
            _serde::export::Err(__err) => {
                return _serde::export::Err(__err);
            }
        };
        match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "f2", &self.f2) {
            _serde::export::Ok(__val) => __val,
            _serde::export::Err(__err) => {
                return _serde::export::Err(__err);
            }
        };
        _serde::ser::SerializeStruct::end(__serde_state)
    }
}
