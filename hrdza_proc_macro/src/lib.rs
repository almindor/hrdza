use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Uff" | "Zle" => "Err",
        "Fajn" | "Ok" => "Ok",
        "Reťazec" => "String",
        "Slovník" => "HashMap",
        "Štandardný" => "Default",
        "Chyba" => "Error",
        "Možno" => "Option",
        "Nejaký" | "Nejaká" | "Nejaké" => "Some",
        "Nič" => "None",
        "Výsledok" => "Result",
        "Ja" => "Self",
        "vypíš" => "print",
        "vypíš_riadok" => "println",
        "preruš" => "break",
        "asynchrónny" | "asynchrónna" | "asynchrónne" => "async",
        "počkaj" => "await",
        "cykli" => "loop",
        "presuň" => "move",
        "krabica" | "v_krabici" => "crate",
        "neprístupny_kód" => "unreachable_code",
        "ako" => "as",
        "konštanta" => "const",
        "vlastnosť" => "trait",
        "nebezpečný" | "nebezpečná" | "nebezpečné" => "unsafe",
        "v" => "in",
        "od" | "z" => "from",
        "dynamický" | "dynamická" | "dynamické" => "dyn",
        "rozbal" => "unwrap",
        "štandardný" | "štandardná" | "štandardné" => "default",
        "ako_odkaz" => "as_ref",
        "es" => "io", // TODO
        "vonkajší" | "vonkajšia" | "vonkajšie" => "extern",
        "falš" => "false",
        "funkcia" => "fn",
        "nad" => "super",
        "vlož" => "insert",
        "daj" => "get",
        "povol" => "allow",
        "prúser" | "panika" | "kurva" => "panic",
        "modul" => "mod",
        "menný" | "menná" | "menné" => "mut",
        "nový" => "new",
        "kde" => "where",
        "pre" => "for",
        "daj_alebo_vlož_s" => "get_or_insert_with",
        "hlavný" | "hlavná" | "hlavné" => "main",
        "verejný" | "verejná" | "verejné" => "pub",
        "čo?" => None?,
        "vráť" => "return",
        "realizuj" => "impl",
        "odkaz" => "ref",
        "zodpovedá" => "match",
        "ak" => "if",
        "inak" => "else",
        "ja" => "self",
        "nech" | "je" => "let",
        "nehybný" | "nehybná" | "nehybné" => "static",
        "štruktúra" | "záznam" => "struct",
        "očakávaj" => "expect",
        "pokiaľ" => "while",
        "použi" => "use",
        "zameň" => "into",
        "pravda" => "true",
        "zoznam" => "enum",
        "štd" => "std",
        "kolekcia" => "collections",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn hrdza(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
