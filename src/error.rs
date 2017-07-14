error_chain! {
    foreign_links {
        IoError(::std::io::Error);
        NomError(::nom::simple_errors::Err);
    }

    errors {
        NoPathGivenError {
            description("No path given!")
            display("No path given!")
        }

        UnexpectedEndError(needed: ::nom::Needed) {
            description("Unexpected end of data!")
            display("Unexpected end of data! Need {} more bytes!", match *needed {
                ::nom::Needed::Size(num) => num.to_string(),
                ::nom::Needed::Unknown => "Unknown".to_string()
            })
        }
    }
}