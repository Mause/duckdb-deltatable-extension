import pandas as pd
from datetime import date
from deltalake.writer import write_deltalake

day = date(2022, 10, 4)
df = pd.DataFrame({
    'x': [1, 2, 3],
    'other': [True, False, True],
    'third': ['foo', 'baz', 'bar'],
    'd': [day, day, day],
})
write_deltalake('test/simple_table', df)
