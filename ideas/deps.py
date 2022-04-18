# Small program for figuring out dependency resolving

# A depends on B
# B depends on C and D
# C depends on D
# D depends on E
deps = {
    "a": ["b"],
    "b": ["c", "d"],
    "c": ["d"],
    "d": ["e", "b"],
    "e": [],
    "f": [],
    "g": []
}
conflicts = {
    "f": ["b"]
}
provides = {
    "g": ["c"]
}

installing = ["a"]

already_installed = []

# step 1: get list of every package to be installed
# recursive algorithm


def get_deps(package, depth):
    depth += 1
    if depth >= 100:
        print('err: maximum recusion depth reached, circular dependency detected')
        exit(0)
    print('getting deps of ' + package)
    pkgs = []
    for dep in deps[package]:
        pkgs.append(dep)
        pkgs += get_deps(dep, depth)
    return pkgs


all_pkgs = []
for pkg in installing:
    if pkg not in already_installed:
        all_pkgs.append(pkg)
        all_pkgs += get_deps(pkg, 0)
print(all_pkgs)

# step 2: resolve conflicts
for pkg in all_pkgs:
    if pkg in conflicts:
        for conflict in conflicts[pkg]:
            if conflict in all_pkgs:
                print(f'{pkg} is in conflict with {conflict}')
                exit(-1)

# step 3: resolve

resolved = []


def dep_resolve(pkg, depth):
    depth += 1
    if depth >= 100:
        print('error: maximum recursion depth reached, most likely a circular dependency')
        exit(0)
    print('resolving ' + pkg)
    if pkg in resolved or pkg in already_installed:
        print('already resolved')
        return
    pkgdeps = deps[pkg]
    if len(pkgdeps) == 0:
        print('no dependencies, adding to the list')
        resolved.append(pkg)
        print('resolved')
        return
    else:
        print('dependencies:', pkgdeps)
        for dep in pkgdeps:
            dep_resolve(dep, depth)
        print('verifying resolution')
        for dep in pkgdeps:
            if dep not in resolved and dep not in already_installed:
                print('resolution failure: missing dependency ' + dep)
                exit(-1)
        print('resolved')
        resolved.append(pkg)


for pkg in all_pkgs:
    dep_resolve(pkg, 0)

print('resolution complete, verifying')
for pkg in all_pkgs:
    if pkg not in resolved and pkg not in already_installed:
        print(f'failed: {pkg} missing from resolved')
        exit(-1)
if len(resolved) == 0:
    print('there is nothing to do')
    exit()
print('finished')
print(resolved)
