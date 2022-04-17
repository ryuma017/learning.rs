use std::marker::PhantomData;
use std::fs::File;

struct  LowerEqualThan500Kb;
struct  LowerEqualThan1000Kb;

struct LimitedSizeFile<T>(File, PhantomData<T>);

struct CsvData;
struct Thumbnail;

impl LimitedSizeFile<LowerEqualThan500Kb> for CsvData {
    // 500 KB 以下に収まって欲しいファイル
}

impl  LimitedSizeFile<LowerEqualThan1000Kb> for Thumbnail {
    // 1000 KB 以下に収まって欲しいファイル
}