/// キューインタフェース
pub trait Queue<T> {
    /// 値xを追加する
    pub fn add(x: T);
    /// 削除する(規則はいろいろある)
    pub fn remove();
}

/// リストインタフェース
pub trait List<T> {
    /// サイズ
    pub fn size() -> usize;
    /// i番目をget
    pub fn get(i: usize);
    /// i番目をupdate
    pub fn set(i: usize, x: T);
    /// i番目として追加し、旧x[i]以降を後ろにずらす
    pub fn add(i: usize, x: T);
    /// i番目を削除し、旧x[i+1]以降を前にずらす
    pub fn remove(i: usize);
}

/// 順序なし集合インタフェース
pub trait USet {
    /// サイズ
    pub fn size() -> usize;
    /// 値xを追加する
    pub fn add(x: T);
    /// 値xを削除する
    pub fn remove(x: T);
    /// 値xを検索する
    pub fn find(x: T) -> Option<T>;
}

/// 順序あり集合インタフェース
pub trait SSet {
    /// サイズ
    pub fn size() -> usize;
    /// 値xを追加する
    pub fn add(x: T);
    /// 値xを削除する
    pub fn remove(x: T);
    /// 値xについて、「y >= xを満たす最少の要素y」を検索する
    pub fn find(x: T) -> Option<T>;
}
