pub use crate::sat::{Backend, GraphDivisionMode, OrderEncodingLinearMode};

// Single source of truth for `bool` config options that have been migrated to the
// macro-based definition below (currently just `use_constant_folding`, as a trial).
// Each row here drives the `Config` struct field, its default in `initial_default()`,
// and its entry in the `bool_flags` table in `parse_from_args()` simultaneously.
// To migrate another option, move its hand-written entry out of `__config_struct_def!`,
// `__config_initial_default_impl!` and `__config_cli_bool_flags!` below and add a row here.
macro_rules! bool_config_options {
    ($mac:ident) => {
        $mac! {
            use_constant_folding: bool = true, doc = "constant folding";
            use_constant_propagation: bool = true, doc = "constant propagation";
            use_norm_domain_refinement: bool = true, doc = "domain refinement in normalized CSP";
            use_direct_encoding: bool = true, doc = "use direct encoding if applicable";
            use_log_encoding: bool = true, doc = "use log encoding if applicable";
            force_use_log_encoding: bool = false, doc = "use log encoding for all int variables";
            use_native_extension_supports: bool = false, doc = "use native propagator for extension (supports) constraints";
            direct_encoding_for_binary_vars: bool = false, doc = "use direct encoding for binary variables";
            merge_equivalent_variables: bool = false, doc = "merge equivalent variables (which is caused by, for example, (iff x y))";
            alldifferent_bijection_constraints: bool = false, doc = "add auxiliary constraints for bijective alldifferent constraints";
            dump_analysis_info: bool = false, doc = "dump analysis info in Glucose";
            glucose_rnd_init_act: bool = false, doc = "rnd_init_act in Glucose";
            optimize_polarity: bool = false, doc = "use polarity-based optimization in decide_irrefutable_facts";
            verbose: bool = false, doc = "show verbose outputs";
        }
    };
}

macro_rules! __config_struct_def {
    ( $( $field:ident : bool = $default:expr, doc = $doc:literal ; )* ) => {
        #[derive(Clone, Copy)]
        pub struct Config {
            $( pub $field: bool, )*
            pub domain_product_threshold: usize,
            pub native_linear_encoding_terms: usize,
            pub native_linear_encoding_domain_product_threshold: usize,
            pub glucose_random_seed: Option<f64>,
            pub backend: Backend,
            pub order_encoding_linear_mode: OrderEncodingLinearMode,
            pub graph_division_mode: GraphDivisionMode,
        }
    };
}
bool_config_options!(__config_struct_def);

macro_rules! __config_initial_default_impl {
    ( $( $field:ident : bool = $default:expr, doc = $doc:literal ; )* ) => {
        impl Config {
            pub fn initial_default() -> Config {
                Config {
                    $( $field: $default, )*
                    domain_product_threshold: 1000,
                    native_linear_encoding_terms: 4,
                    native_linear_encoding_domain_product_threshold: 20,
                    glucose_random_seed: None,
                    backend: default_backend_from_env(),
                    order_encoding_linear_mode: OrderEncodingLinearMode::Cpp,
                    graph_division_mode: GraphDivisionMode::Cpp,
                }
            }
        }
    };
}
bool_config_options!(__config_initial_default_impl);

thread_local! {
    static DEFAULT_CONFIG: std::cell::Cell<Config> = {
        std::cell::Cell::new(Config::initial_default())
    };
}

fn parse_backend(s: &str) -> Option<Backend> {
    if s == "glucose" {
        Some(Backend::Glucose)
    } else if s == "glucose_rs" {
        #[cfg(feature = "experimental-backend-glucose-rs")]
        {
            Some(Backend::GlucoseRs)
        }
        #[cfg(not(feature = "experimental-backend-glucose-rs"))]
        {
            None
        }
    } else if s == "external" {
        Some(Backend::External)
    } else if s == "cadical" {
        Some(Backend::CaDiCaL)
    } else {
        None
    }
}

#[cfg(target_arch = "wasm32")]
fn default_backend_from_env() -> Backend {
    // In wasm, we cannot use environment variables, so we just return the default backend.
    Backend::Glucose
}

#[cfg(not(target_arch = "wasm32"))]
fn default_backend_from_env() -> Backend {
    if let Ok(s) = std::env::var("CSPUZ_CORE_DEFAULT_BACKEND") {
        parse_backend(&s).unwrap_or_else(|| {
            panic!("error: unknown backend specified in CSPUZ_CORE_DEFAULT_BACKEND");
        })
    } else {
        Backend::Glucose
    }
}

fn to_config_name(s: &str) -> String {
    s.replace('-', "_")
}

impl Config {
    pub fn default() -> Config {
        DEFAULT_CONFIG.with(|f| f.get())
    }

    pub fn set_default(new_default: Config) {
        DEFAULT_CONFIG.with(|f| f.set(new_default));
    }

    #[cfg(feature = "cli")]
    pub fn parse_from_args() -> Config {
        use getopts::Options;
        use std::str::FromStr;

        let args = std::env::args().collect::<Vec<_>>();
        let mut config = Config::default();
        let mut opts = Options::new();

        macro_rules! __config_cli_bool_flags {
            ( $( $field:ident : bool = $default:expr, doc = $doc:literal ; )* ) => {
                [
                    $( (&mut config.$field, to_config_name(stringify!($field)), $doc), )*
                ]
            };
        }
        let mut bool_flags = bool_config_options!(__config_cli_bool_flags);
        for (opt, name, desc) in &mut bool_flags {
            if **opt {
                opts.optflag(
                    "",
                    &format!("enable-{}", name),
                    &format!("Enable {} (default).", desc),
                );
                opts.optflag(
                    "",
                    &format!("disable-{}", name),
                    &format!("Disable {}.", desc),
                );
            } else {
                opts.optflag(
                    "",
                    &format!("enable-{}", name),
                    &format!("Enable {}.", desc),
                );
                opts.optflag(
                    "",
                    &format!("disable-{}", name),
                    &format!("Disable {} (default).", desc),
                );
            }
        }
        opts.optopt("", "domain-product-threshold", "Specify the threshold of domain product for introducing an auxiliary variable by Tseitin transformation.", "THRESHOLD");
        opts.optopt("", "native-linear-encoding-terms", "Specify the maximum number of terms in a linear sum which is encoded by the native linear constraint (0 for disabling this).", "TERMS");
        opts.optopt("", "native-linear-encoding-domain-product", "Specify the minimum domain product of linear sums which are encoded by the native linear constraint.", "DOMAIN_PRODUCT");

        opts.optopt("", "backend", "Specify the SAT backend", "BACKEND");
        opts.optopt(
            "",
            "order-encoding-linear-mode",
            "Specify the implementation kind used for native linear constraints",
            "MODE",
        );

        opts.optflag("h", "help", "Display this help");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => {
                println!("error: {}", f);
                std::process::exit(1);
            }
        };

        if matches.opt_present("h") {
            // display help
            let brief = format!("Usage: {} [options]", args[0]);
            print!("{}", opts.usage(&brief));
            std::process::exit(0);
        }

        for (opt, name, _) in &mut bool_flags {
            let is_set_enable = matches.opt_present(&format!("enable-{}", name));
            let is_set_disable = matches.opt_present(&format!("disable-{}", name));

            match (is_set_enable, is_set_disable) {
                (true, true) => {
                    println!(
                        "error: conflicting options {} and {} are specified at the same time",
                        name, name
                    );
                    std::process::exit(1);
                }
                (true, false) => **opt = true,
                (false, true) => **opt = false,
                (false, false) => (),
            }
        }

        fn maybe_set_option<T: FromStr>(matches: &getopts::Matches, store: &mut T, arg_name: &str) {
            if let Some(s) = matches.opt_str(arg_name) {
                match s.parse::<T>() {
                    Ok(v) => *store = v,
                    Err(_) => {
                        println!("error: parse failed for --{}: {}", arg_name, s);
                        std::process::exit(1);
                    }
                }
            }
        }

        maybe_set_option(
            &matches,
            &mut config.domain_product_threshold,
            "domain-product-threshold",
        );
        maybe_set_option(
            &matches,
            &mut config.native_linear_encoding_terms,
            "native-linear-encoding-terms",
        );
        maybe_set_option(
            &matches,
            &mut config.native_linear_encoding_domain_product_threshold,
            "native-linear-encoding-domain-product",
        );

        if let Some(s) = matches.opt_str("backend") {
            config.backend = parse_backend(&s).unwrap_or_else(|| {
                println!("error: unknown backend: {}", s);
                std::process::exit(1);
            });
        }
        if let Some(s) = matches.opt_str("order-encoding-linear-mode") {
            if s == "cpp" {
                config.order_encoding_linear_mode = OrderEncodingLinearMode::Cpp;
            } else if s == "rust" {
                config.order_encoding_linear_mode = OrderEncodingLinearMode::Rust;
            } else if s == "rust-optimized" {
                config.order_encoding_linear_mode = OrderEncodingLinearMode::RustOptimized;
            } else {
                println!("error: unknown linear implementation: {}", s);
                std::process::exit(1);
            }
        }

        config
    }
}
