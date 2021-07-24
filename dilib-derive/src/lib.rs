mod target;
mod dependency;
mod constructor;
mod helpers;
mod utils;
mod strings;
mod error;

use proc_macro::TokenStream;
use crate::target::parse_derive_injectable;

#[proc_macro_derive(Injectable, attributes(inject))]
pub fn derive_injectable_attribute(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item);

    parse_derive_injectable(input)
        .emit()
        .into()
}

/*
#[derive(Injectable)]
#[inject(constructor="new(api_key, count, ...)")]
struct ProductService {
    http_client: Singleton<HttpClient>,
    logger: Logger,

    #[inject(name="API_KEY")]
    api_key: String,

    #[inject(scope="singleton")]
    id: Singleton<usize>,

    #[inject(scope="scoped")]
    client_service: ClientService,

    #[inject(default=1)]
    count: usize,
}

struct InjectableTarget {
    target_type: Box<Type>,
    constructor: Option<TargetConstructor>,
    deps: Vec<Dependency>,
}

struct TargetConstructor {
   name: String,
   params: Vec<String>
}

struct Dependency {
    field: Field,
    field_type: Box<Type>,
    scope: Scope,

    name: Option<String>,
    default_value: Option<Literal>
 }

enum Field {
   Named(String),
   Unnamed(usize)
}
*/

/*
#[derive(Injectable))
struct Service(
    ClientService,
    Singleton<usize>
);

impl Injectable for Service {
    fn resolve(container: &Container) -> Self {
        let _0 = container.get_scoped::<ClientService>().expect("Cannot find dependency of type 'ClientService'");
        let _1 = container.get_singleton::<usize>().expect('Cannot find singleton of type 'usize');
        Service(_0, _1)
    }
}
*/

/*
trait Module {
    fn get_all(self) -> Vec<(InjectionKey, Provider)>;
}

#[provide(scope="singleton", name="my")]
struct MyService {
    n: usize
}

#[provide(name="API_KEY")]
static API_KEY : Singleton<String> = Singleton::new("asa2fw2r9qej3qs9");

#[provide(name="RANDOM")]
fn random_number() -> usize {
    Rng::new().gen()
}

struct MySome {
   #[inject(name="RANDOM")]
   value: usize
}

// container.add_deps_with_name::<MyService>("my");

fn main() {
    let mut container = Container::new();
    // container.add_module(MyModule::new());

    container.add_scoped_default::<HttpClient>();
    container.add_singleton::<usize>(0);
    container.add_scoped_with_name::<String>("API_KEY", String::new("ff1c75da5ad6258b87a1"));
    container.add_deps::<ClientService>();
    container.add_deps::<ProductService>();

    let server = Server::new('localhost:5000');
    loop {
       let (req, res) = server.listen();

       if req.url.contains("/clients") {
          let clients = container.get_scoped::<ClientService>().unwrap();
          clients.handle(req, res);
       }

       if req.url.contains("/products") {
          let products = container.get_scoped::<ProductService>().unwrap();
          products.handle(req, res);
       }
    }
}
*/