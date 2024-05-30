import polars as pl

from rdkit_canonicalizer import canonicalize

# from rdkit_canonicalizer import pig_latinnify


# df_pig_latin = pl.DataFrame({
#     'english': ['this', 'is', 'not', 'pig', 'latin'],
# })
# result = df_pig_latin.with_columns(pig_latin = pig_latinnify('english'))
# print(result)

df = pl.DataFrame({
    'sml': ["C1=CC=CC=C1", "C1C=CC=CC=1"],
})
result = df.with_columns(can_sml = canonicalize('sml'))
print(result)

# Expected result:
# ['c1ccccc1', 'c1ccccc1']
