# hrdza

![](https://github.com/almindor/hrdza/raw/master/logo.jpeg)

Aren't you _unavený_ from writing Rust programs in English? Do you like saying
"kurva" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Slovak touch to your
programs?

**hrdza** (Slovak for _Rust_) is here to save your day, as it allows you to
write Rust programs in Slovak, using Slovak keywords, Slovak function names,
Slovak idioms.

This has been designed to be used as the official programming language to
develop the future Slovak sovereign operating system. If you're from the SIS: I will be awaiting your donations on
[liberapay](https://liberapay.com/almindor/).

You're from Slovakia and don't feel at ease using only Slovak words? Don't worry!
Slovak Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Hrdza:

### trait and impl (aka convention et réalisation)

```rust
hrdza::hrdza! {
    vonkajšia krabica hrdza;

    použi štd::kolekcia::Slovník ako Slovník;

    vlastnosť KlúčHodnota {
        funkcia zapíš(&ja, kľúč: Reťazec, hodnota: Reťazec);
        funkcia čítaj(&ja, kľúč: Reťazec) -> Výsledok<Možno<&Reťazec>, Reťazec>;
    }

    nehybný menný SLOVNÍK: Možno<Slovník<Reťazec, Reťazec>> = Nič;

    // Sľúb mi, že toto nikdy nepoužiješ mimo hlavné vlákno!
    štruktúra GlobálnySlovník;

    realizuj KlúčHodnota pre GlobálnySlovník {
        funkcia zapíš(&ja, kľúč: Reťazec, hodnota: Reťazec) {
            nech slovník = nebezpečné {
                SLOVNÍK.daj_alebo_vlož_s(Štandardný::štandardný)
            };

            slovník.vlož(kľúč, hodnota);
        }

        funkcia čítaj(&ja, kľúč: Reťazec) -> Výsledok<Možno<&Reťazec>, Reťazec> {
            ak je Nejaký(slovník) = nebezpečný { SLOVNÍK.ako_odkaz() } {
                Fajn(slovník.daj(&kľúč))
            } inak {
                Zle("Overenie slovníku".zameň())
            }
        }
    }
}
```

### Support for swear words

```rust
    #[povol(neprístupny_kód)]
    funkcia keď_sa_nepodarí() {
        panika!("Panika!");
        prúser!("Niečo sa pokazilo");
        kurva!("Celé zle!");
    }
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. _Tak a je to_, that's it.

## príspevky

First of all, _pekne ďakujem_ for considering participating to this joke, the
SIS will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `hlavna` (Slovak for
`main`) branch.

In the Slovak spirit, feel free to introduce swear words.

## But prečo would you do this?

- horsin around
- playing with raw proc macros
- making a bit of fun about programming languages that do this seriously,
  though I can see their utility.

## Other languages

Here's a non-exhaustive list of implementations for other languages:

* Czech: [red](https://github.com/radekvit/rez)
* Romanian: [rugina](https://github.com/aionescu/rugina)
* French: [rouille](https://github.com/bnjbvr/rouille)
* Dutch: [roest](https://github.com/jeroenhd/roest)
* German: [rost](https://github.com/michidk/rost)
* Polish: [rdza](https://github.com/phaux/rdza)
* Italian: [ruggine](https://github.com/DamianX/ruggine)
* Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
* Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
* Hindi: [zung](https://github.com/rishit-khandelwal/zung)
* Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
* Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
* Spanish: [oxido](https://github.com/fdschonborn/oxido)
* Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
* Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
* Arabic: [sada](https://github.com/LAYGATOR/sada)
* Turkish: [pas](https://github.com/ekimb/pas)
* Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
* Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
* Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
* Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)

You can find a more comprehensive list on [@bnjbvr](https://github.com/bnjbvr)'s [rouille](https://github.com/bnjbvr/rouille#other-languages) repo.

## _Poďakovanie_

* [@bnjbvr](https://github.com/bnjbvr) for the idea, as well as the base repo

## Licencia

[WTFPL](http://www.wtfpl.net/)