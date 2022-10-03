import pandas as pd
from deltalake.writer import write_deltalake
df = pd.DataFrame({
    'x': [1, 2, 3],
    'other': [True, False, True],
    'third': ['foo', 'baz', 'bar']
})
write_deltalake('test/simple_table', df)
