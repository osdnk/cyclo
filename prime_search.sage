# ===========================
# Cyclotomic scan & pretty print
# SageMath / Python (Sage) code
# ===========================

# ---- Parameters you can tweak ----
DEG_MIN = 16             # require φ(n) >= 16
DEG_MAX = 512            # require φ(n) <= 512
MAX_TERMS = 10           # "fewer than 10" nonzero coefficients (strict)
N_MAX = 100000           # search up to this n
K_MIN = 2                # evaluate Φ_n(k) for k in [K_MIN, K_MAX]
K_MAX = 1000
MAX_BIT_LENGTH = 1024    # require deg(Φ_n) * bit_length(k) <= MAX_BIT_LENGTH
PRIMALITY_PROOF = True   # ask Sage for a proven primality test
SHOW_CANDIDATE_TABLE = False  # print the (deg, n, terms) table before scanning k
# ----------------------------------

R.<x> = PolynomialRing(ZZ)

def nonzero_term_count(f):
    """Number of monomials with nonzero coefficient."""
    return len(f.dict())

def divider(label, char="─"):
    line = char * max(3, len(label))
    return f"{label}\n{line}"

def format_poly_subst(poly, k):
    """
    Return a readable string for poly(k). Example:
        x^8 - x^4 + 1  -->  2^8 - 2^4 + 1  if k=2
    """
    # Replace "x" safely with the integer k; parentheses not needed since k >= 2
    return str(poly).replace("x", str(k))

def collect_candidates():
    """
    Collect cyclotomic polynomials Φ_n(x) with:
      - DEG_MIN <= φ(n) <= DEG_MAX
      - nonzero terms < MAX_TERMS
    Returns a list of tuples (deg, n, terms, poly), sorted by (deg, n).
    """
    rows = []
    for n in range(1, N_MAX + 1):
        deg = euler_phi(n)
        if DEG_MIN <= deg <= DEG_MAX:
            f = cyclotomic_polynomial(n)
            k_terms = nonzero_term_count(f)
            if k_terms < MAX_TERMS:                  # strict: "fewer than 10"
                rows.append((deg, n, k_terms, f))
    rows.sort(key=lambda t: (t[0], t[1]))
    return rows

def print_candidate_table(rows, max_rows=200):
    """Pretty-print a compact table of candidate Φ_n."""
    print(divider("Candidate cyclotomic polynomials Φ_n(x)"))
    print(f"Total candidates: {len(rows)}")
    print()
    headers = ("deg", "n", "terms", "Φ_n(x)")
    # Compute column widths (excluding polynomial column)
    deg_w = max(len(str(r[0])) for r in rows) if rows else 3
    n_w   = max(len(str(r[1])) for r in rows) if rows else 1
    t_w   = max(len(str(r[2])) for r in rows) if rows else 5
    print(f"{headers[0]:>{deg_w}}  {headers[1]:>{n_w}}  {headers[2]:>{t_w}}  {headers[3]}")
    print("-" * (deg_w + n_w + t_w + 6 + 40))
    shown = 0
    for deg, n, terms, poly in rows:
        print(f"{deg:>{deg_w}}  {n:>{n_w}}  {terms:>{t_w}}  {poly}")
        shown += 1
        if shown >= max_rows and len(rows) > max_rows:
            print(f"... ({len(rows) - max_rows} more not shown)")
            break
    print()

def find_prime_values(rows):
    """
    For each candidate Φ_n(x) and each integer k in [K_MIN..K_MAX],
    evaluate q = Φ_n(k). If q is prime and deg * bitlen(k) <= MAX_BIT_LENGTH,
    record it. Returns a dict: { prime q : [human_readable_descriptions...] }.
    """
    primes = {}
    total_checked = 0

    print(divider("Scanning k for prime values"))
    print(f"k range       : {K_MIN} .. {K_MAX}")
    print(f"bit-length cap: deg(Φ_n) * bitlen(k) <= {MAX_BIT_LENGTH}")
    print()

    for k in range(K_MIN, K_MAX + 1):
        for deg, n, _terms, poly in rows:
            # quick bit-length bound
            if deg * int(k).bit_length() > MAX_BIT_LENGTH:
                continue

            q = poly(k)
            # Skip nonpositive values

            if ZZ(q).is_prime(proof=PRIMALITY_PROOF):
                expr = f"{format_poly_subst(poly, k)}   (Φ_{n}({k}))"
                primes.setdefault(q, []).append(expr)

            total_checked += 1

    print()
    print(f"Done. Evaluated {total_checked} values of Φ_n(k).")
    print(f"Found {len(primes)} distinct prime values.")
    print()
    return primes

def print_prime_report(primes):
    """
    Pretty-print the primes found, grouped by bit-length and ordered nicely.
    """
    if not primes:
        print("No primes found under the given constraints.")
        return

    # Sort by (bit_length, numeric value)
    def bl(x): 
        try:
            return int(ZZ(x).nbits())
        except Exception:
            return int(int(x).bit_length())

    items = sorted(primes.items(), key=lambda kv: (bl(kv[0]), kv[0]))
    current_bits = None

    print(divider("Prime hits"))
    for p, exprs in items:
        bits = bl(p)
        if bits != current_bits:
            current_bits = bits
            print()
            print(f"## {bits}-bit primes")
            print()

        print(f"{p} =")
        for ex in exprs:
            print(f"    {ex}")
        print()

# ===== Main flow =====
print(divider("Search parameters"))
print(f"φ(n) range    : [{DEG_MIN}, {DEG_MAX}]")
print(f"n range       : 1 .. {N_MAX}")
print(f"term limit    : < {MAX_TERMS} nonzero monomials")
print(f"primality     : {'proof-based' if PRIMALITY_PROOF else 'probabilistic'}")
print()

candidates = collect_candidates()

if SHOW_CANDIDATE_TABLE:
    print_candidate_table(candidates)

primes = find_prime_values(candidates)
print_prime_report(primes)
