struct App {
    context: Context,
}

struct Context {
    id: Id,
    param: Param,
}

#[derive(Copy, Clone)]
struct Id(u64);
impl FromContext for Id {
    fn extract(context: &Context) -> Self {
        context.id
    }
}

#[derive(Clone)]
struct Param(String);
impl FromContext for Param {
    fn extract(context: &Context) -> Self {
        context.param.clone()
    }
}

impl App {
    fn new() -> Self {
        Self {
            context: Context {
                id: Id(23232),
                param: Param(format!("Hello, world!")),
            },
        }
    }

    fn route<F: Handler<T>, T: FromContext>(self, path: impl AsRef<str>, handler: F) -> Self {
        handler.handle(&self.context);
        self
    }
}

trait FromContext {
    fn extract(context: &Context) -> Self;
}

impl FromContext for () {
    fn extract(_context: &Context) -> Self {
        ()
    }
}

trait Handler<T> {
    fn handle(self, context: &Context);
}

impl<F, T> Handler<T> for F
where
    F: Fn(T),
    T: FromContext,
{
    fn handle(self, context: &Context) {
        self(T::extract(context))
    }
}

#[test]
fn test_app() {
    let app = App::new().route("/", |_: ()| {
        println!("Hello, world!");
    });
}
