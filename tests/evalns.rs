use fasteval::ez_eval;

#[test]
fn empty() {
    let mut ns = fasteval::EmptyNamespace;

    let val = ez_eval("1 + 1", &mut ns).unwrap();
    assert_eq!(val, 2.0);
}

#[test]
fn str_to_f64() {
    {
        let mut ns = fasteval::StringToF64Namespace::new();
        ns.insert("a".to_string(), 1.11);
        ns.insert("b".to_string(), 2.22);

        let val = ez_eval("a + b + 1", &mut ns).unwrap();
        assert_eq!(val, 4.33);
    }

    {
        let mut ns = fasteval::StrToF64Namespace::new();
        ns.insert("a", 1.11);
        ns.insert("b", 2.22);

        let val = ez_eval("a + b + 1", &mut ns).unwrap();
        assert_eq!(val, 4.33);
    }
}

#[test]
fn str_to_cb() {
    {
        let mut ns = fasteval::StringToCallbackNamespace::new();
        ns.insert("a".to_string(), Box::new(|args| args[0]));
        ns.insert("b".to_string(), Box::new(|args| args[0] * 2.0));

        let val = ez_eval("a(1.11) + b(1.11) + 1", &mut ns).unwrap();
        assert_eq!(val, 4.33);
    }

    {
        let mut ns = fasteval::StrToCallbackNamespace::new();
        ns.insert("a", Box::new(|args| args[0]));
        ns.insert("b", Box::new(|args| args[0] * 2.0));

        let val = ez_eval("a(1.11) + b(1.11) + 1", &mut ns).unwrap();
        assert_eq!(val, 4.33);
    }
}

#[test]
fn layered_str_to_f64() {
    let mut ns = fasteval::LayeredStringToF64Namespace::new();
    let mut layer0 = fasteval::StringToF64Namespace::new();
    layer0.insert("a".to_string(), 1.11);
    layer0.insert("b".to_string(), 2.22);
    ns.push(layer0);

    let val = ez_eval("a + b + 1", &mut ns).unwrap();
    assert_eq!(val, 4.33);

    let mut layer1 = fasteval::StringToF64Namespace::new();
    layer1.insert("a".to_string(), 11.11);
    ns.push(layer1);

    let val = ez_eval("a + b + 1", &mut ns).unwrap();
    assert_eq!(val, 14.33);

    ns.pop();

    let val = ez_eval("a + b + 1", &mut ns).unwrap();
    assert_eq!(val, 4.33);
}

#[test]
fn cb() {
    let mut ns = |name:&str, args:Vec<f64>| {
        match name {
            "a" => Some(1.11),
            "b" => Some(2.22),
            "len" => Some(args.len() as f64),
            _ => None,
        }
    };

    let val = ez_eval("a + b + 1", &mut ns).unwrap();
    assert_eq!(val, 4.33);
}

#[test]
fn cached_cb() {
    let mut ns = fasteval::CachedCallbackNamespace::new(|name:&str, args:Vec<f64>| {
        match name {
            "a" => { eprintln!("cached_cb: a: This should only be printed once."); Some(1.11) }
            "b" => Some(2.22),
            "len" => Some(args.len() as f64),
            _ => None,
        }
    });

    let val = ez_eval("a + b + 1", &mut ns).unwrap();
    assert_eq!(val, 4.33);
    ez_eval("a + b + 1", &mut ns).unwrap();
    ez_eval("a + b + 1", &mut ns).unwrap();
    ez_eval("a + b + 1", &mut ns).unwrap();
    ez_eval("a + b + 1", &mut ns).unwrap();
}

#[test]
fn custom_vector_funcs() {
    let vecs_cell = std::cell::RefCell::new(Vec::<Vec<f64>>::new());

    let mut ns = fasteval::StrToCallbackNamespace::new();

    ns.insert("x", Box::new(|_args| 2.0));

    ns.insert("vec_store", Box::new(|args| {
        let mut vecs = vecs_cell.borrow_mut();
        let index = vecs.len();
        vecs.push(args);
        index as f64
    }));

    ns.insert("vec_sum", Box::new(|args| {
        if let Some(index) = args.get(0) {
            if let Some(v) = vecs_cell.borrow().get(*index as usize) {
                return v.iter().sum();
            }
        }
        std::f64::NAN
    }));

    let val = ez_eval("vec_sum(vec_store(1.1, x, 3.3)) + vec_sum(0)", &mut ns).unwrap();
    assert_eq!(val, 12.8);
}

