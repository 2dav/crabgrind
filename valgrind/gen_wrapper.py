CLIENT_REQUESTS_DEFS_PATH = "client_request.defs"
WRAPPER_PATH = "wrapper.h"
CLIENT_REQ_PLACEHOLDER = "{{% client_request_defs %}}"
VALGRIND_LOWEST_CLIENT_REQUEST = 4 << 10
TAB = "  "
NEWLINE = "\n"


def tool_name(tool):
    return f"CG_{tool}ClientRequest"


def variant_name(api):
    return f"CG_{api.split("VG_")[1]}"

    
def variant(api, n):
    assert n < VALGRIND_LOWEST_CLIENT_REQUEST
    vname = variant_name(api)
    return f"""#ifdef {api}
{TAB}{vname} = {api},
#else
{TAB}{vname} = {n},
#endif"""


def variants(apis):
    return [variant(api, n) for n, api in enumerate(apis)]


def enum(tool, apis):
    return f"""
typedef enum {{
{NEWLINE.join(variants(apis))}
}} {tool_name(tool)};"""


def gen_defs():
    with open(CLIENT_REQUESTS_DEFS_PATH, 'r') as f:
        tool = None
        apis = []
        buf = ""
    
        for line in f:
            line = line.strip()
    
            if not line:
                continue
    
            if line.endswith(":"):
                if tool:
                    buf += enum(tool, apis) + NEWLINE
    
                tool = line[:-1]
                apis.clear()
            else:
                apis.append(line)
    return buf


def gen_wrapper():
    defs = gen_defs()
    with open(WRAPPER_PATH, 'r') as f:
        return f.read().replace(CLIENT_REQ_PLACEHOLDER, defs)


print(gen_wrapper())
