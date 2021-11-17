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

    verejná(v_krabici) funkcia možno(i: u32) -> Možno<Výsledok<u32, Reťazec>> {
        ak i % 2 == 1 {
            ak i == 42 {
                Nejaký(Uff(Reťazec::z("Juj!")))
            } inak {
                Nejaký(Fajn(33))
            }
        } inak {
            Nič
        }
    }

    asynchrónna funkcia ukážka() { }

    asynchrónna funkcia ukážka2() {
        ukážka().počkaj;
    }

    funkcia hlavná() {
        nech menné x = 31;

        zodpovedá x {
            42 => {
                vypíš_riadok!("Význam vesmíru je {}!", x)
            }
            _ => vypíš_riadok!("Tak nič.")
        }

        pre i v 0..10 {
            nech hodnota = cykli {
                preruš i;
            };

            pokiaľ x < hodnota {
                x += 1;
            }

            x = ak je Nejaký(výsledok) = možno(i) {
                výsledok.rozbal()
            } inak {
                12
            };
        }
    }

    #[povol(neprístupny_kód)]
    funkcia keď_sa_nepodarí() {
        panika!("Panika!");
        prúser!("Niečo sa pokazilo");
    }
}