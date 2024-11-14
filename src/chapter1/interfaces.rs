/// キューインタフェース
pub trait Queue<T> {
    /// 値xを追加する
    fn add(&self, x: T);
    /// 削除する(規則はいろいろある)
    fn remove(&self) -> T;
}

/// リストインタフェース
pub trait List<T> {
    /// サイズ
    fn size(&self) -> usize;
    /// i番目をget
    fn get(&self, i: usize);
    /// i番目をupdate
    fn set(&self, i: usize, x: T);
    /// i番目として追加し、旧x[i]以降を後ろにずらす
    fn add(&self, i: usize, x: T);
    /// i番目を削除し、旧x[i+1]以降を前にずらす
    fn remove(&self, i: usize);
}

/// 順序なし集合インタフェース
pub trait USet<T> {
    /// サイズ
    fn size(&self) -> usize;
    /// 値xを追加する
    fn add(&mut self, x: T) -> bool;
    /// 値xを削除する
    fn remove(&mut self, x: T) -> Option<T>;
    /// 値xを検索する
    fn find(&self, x: T) -> Option<T>;
}

/// 順序あり集合インタフェース
pub trait SSet<T> {
    /// サイズ
    fn size(&self) -> usize;
    /// 値xを追加する
    fn add(&mut self, x: T) -> bool;
    /// 値xを削除する
    fn remove(&mut self, x: T) -> Option<T>;
    /// 値xについて、「y >= xを満たす最少の要素y」を検索する
    fn find(&self, x: T) -> Option<T>;
}
