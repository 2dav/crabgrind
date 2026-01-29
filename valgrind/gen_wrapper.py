import sys
from dataclasses import dataclass
from typing import Set
import os

CLIENT_REQUESTS_DEFS_PATH = "client_request.defs"
WRAPPER_HEAD = "wrapper.head"
TAB = "\t"
NEWLINE = "\n"
PREFIX = "CG_"


def tool_name(tool):
    return f"{PREFIX}{tool.strip()}ClientRequest"


def variant_name(vname):
    return f"{PREFIX}{vname.strip()}"


def r_impl_prolog(name):
    return f"""impl {name} {{
    pub const fn required_version(self) -> u32 {{
        match self {{"""


def c_enum_epi(tool):
    return f"}} {tool};"


def c_enum_variant_def(api, vname, vvar, n):
    return f"""#ifdef {api}
{TAB}{vname} = {vvar},
#else
{TAB}{vname} = {n},
#endif"""


def variant_info(api):
    vname, vvar = api.split("=")
    vvar, version = vvar.split(",")
    return vname, variant_name(vname), vvar.strip(), version.strip()

    
def variant(definition, cache):
    api, vname, vvar, vver = variant_info(definition)

    assert api not in cache, f"duplicate API {api}"
    assert vvar not in cache, f"duplicate const {vvar}"
    assert int(vver) > 0, f"{vver} missing minimum required valgrind version for {api}"
    cache.update([api, vvar])

    return api, vname, vvar, vver


@dataclass
class Gen:
    history: Set
    current_tool: str = ""
    c_enums: str = ""
    r_impls: str = ""
    n: int = 0

    def append_enum(self, name: str):
        if self.current_tool:
            self.c_enums, self.r_impls = self.finish()

        self.n = 0;
        self.current_tool = tool_name(name)
        self.c_enums += NEWLINE + "typedef enum {" + NEWLINE
        self.r_impls += r_impl_prolog(self.current_tool) + NEWLINE

    def append_variant_def(self, definition: str):
        api, vname, vvar, vver = variant(definition, self.history)
        self.c_enums += c_enum_variant_def(api, vname, vvar, self.n) + NEWLINE
        self.r_impls += f"""{TAB}{TAB}{TAB}Self::{vname} => {vver},""" + NEWLINE
        self.n += 1

    def finish(self):
        return self.c_enums + c_enum_epi(self.current_tool) + NEWLINE, \
                self.r_impls + f"{TAB}{TAB}}}\n{TAB}}}\n}}" + NEWLINE


def gen_defs():
    with open(CLIENT_REQUESTS_DEFS_PATH, 'r') as f:
        gen = Gen(history=set())
    
        for line in f:
            line = line.strip()
    
            if not line:
                continue
    
            if line.endswith(":"):
                gen.append_enum(line[:-1])
            else:
                gen.append_variant_def(line)

        return gen.finish()


def gen_wrapper():
    cdefs, rdefs = gen_defs()
    with open(WRAPPER_HEAD, 'r') as f:
        return f.read() + cdefs, rdefs


if len(sys.argv) < 3:
    print("Usage: gen_wrapper.py <header output> <versions output>")
    sys.exit(1)


wrapper_out = sys.argv[1]
versions_out = sys.argv[2]

c, r = gen_wrapper()

with open(wrapper_out, 'w') as f:
    f.write(c)

with open(versions_out, 'w') as f:
    f.write(r)

print("C definitions > ", os.path.join(os.getcwd(), wrapper_out))
print("Rust impls > ", os.path.join(os.getcwd(), versions_out))
