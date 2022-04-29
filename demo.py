import man_parse_rust


def manchester_parse_demo():
    # man_string = "<obo>:OBI_0000070 SubClassOf: obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO:0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))))"
    man_string = "obo:OBI_0000070 SubClassOf: obo:OBI_0000299 some (obo:IAO_0000027 and (obo:IAO:0000136 only (obo:BFO_0000040 or (not (obo:RO_0000087 some obo:OBI_0000067)))))"
    syntax_check = man_parse_rust.syntax_check(man_string)
    errors = man_parse_rust.get_syntax_errors(man_string)
    ofn = man_parse_rust.get_ofn(man_string)

    print("Syntax Check passed: " + str(syntax_check))
    print("Errors are: " + str(errors))
    print("OFN is: " + ofn)


if __name__ == "__main__":
    manchester_parse_demo()
