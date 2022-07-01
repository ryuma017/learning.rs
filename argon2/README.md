Webアプリケーションのセキュリティに関する資料の宝庫である Open Web Application Security Project (OWASP) が公開しているチートシートシリーズのパスワードの安全な保存方法に関するセクションで推奨されているアルゴリズム。

> ・Use Argon2id with a minimum configuration of 15 MiB of memory, an iteration count of 2, and 1 degree of parallelism.
> ・If Argon2id is not available, use scrypt with a minimum CPU/memory cost parameter of (2^16), a minimum block size of 8 (1024 bytes), and a parallelization parameter of 1.
> ・For legacy systems using bcrypt, use a work factor of 10 or more and with a password limit of 72 bytes.
> ・If FIPS-140 compliance is required, use PBKDF2 with a work factor of 310,000 or more and set with an internal hash function of HMAC-SHA-256.
> ・Consider using a pepper to provide additional defense in depth (though alone, it provides no additional secure characteristics).

# Argon2 Hashing

## インスタンスを作成
パスワードをハッシュ化するためには、まず `Argon2` 構造体のインスタンスを作成する必要がある。
`Argon2::new` のシグネチャは以下。

```rust
impl<'key> Argon2<'key> {
    pub fn new(algorithm: Algorithm, version: Version, params: Params) -> Self {/* */}
}
```

`Algorithm` と `Version` はそれぞれ `enum` で定義されている。

```rust
pub enum Algorithm {
    Argon2d,
    Argon2i,
    Argon2id,
}

#[repr(u32)]
pub enum Version {
    V0x10,
    V0x13,
}
```

Params は ビルドに必要なパラメータをフィールドに持つ `struct`。
`Params::new` のシグネチャは以下。

```rust
/// Create new parameters.
pub fn new(
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
    output_len: Option<usize>
) -> Result<Self>
{/* ... */}
```

引数の3つのパラメータはそれぞれ、メモリサイズ(m: memory size)、反復回数(t: number of iterations)、並列度(p: egree of parallelism)に対応している。
`output_len` は返されるハッシュの長さ。デフォルトでは 32bytes になる。

## Salting
`Argon2` は [`password-hash`](https://docs.rs/password-hash/latest/password_hash/index.html)から re-export された、Argon2やscryptなどのパスワードハッシュを扱うための統一されたインターフェースである`PasswordHasher` を実装している。
`PasswordHasher::hash_password` は引数として、生のパスワードと `salt` を要求する。

```rust
pub trait PasswordHasher {
    // ...
    fn hash_password<'a, S>(
        &self,
        password: &[u8],
        salt: &'a S
    ) -> Result<PasswordHash<'a>>
    where
        S: AsRef<str> + ?Sized,
    {/* ... */}
}
```

Salt とは、各パスワードに対して割り当てられる一意のランダムな文字列。
その割り当てる行為を塩付け(Salting)と呼ぶ。詳解は[ここ](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#salting)

`SaltString::generate()` と `rand::thread_rng()` を使用し、以下のように生成する。

```rust
use argon2::password_hash::SaltString;

let salt = SaltString::generate(&mut rand::thread_rng());
```

## PHC string format
[PHC string format](https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md) とは、ハッシュ化されたパスワードそのものと、salt、アルゴリズムなどの関連するパラメータの情報を含む、パスワードハッシュの標準的な表現を提供する文字列フォーマット。

```
$<id>[$v=<version>][$<param>=<value>(,<param>=<value>)*][$<salt>[$<hash>]]
```

`argon2` は PHC string format のRust実装である、`PasswordHash` を提供しており、上述の `PasswordHasher::hash_password`の返り値はそれを `Result` で wrap したもの。

```rust
pub struct PasswordHash<'a> {
    pub algorithm: Ident<'a>,
    pub version: Option<Decimal>,
    pub params: ParamsString,
    pub salt: Option<Salt<'a>>,
    pub hash: Option<Output>,
}
```

## Hashing
以上を踏まえて、生のパスワードを受け取り、ハッシュ化されたパスワードを返す関数を書く。

今回はOWASPの勧告に従い、

- アルゴリズム: `Algorithm::Argon2id`
- バージョン: `Version::V0x13`
- パラメータ:
    - `m_cost`: 15 MiB
    - `t_cost`: 2
    - `p_cost`: 1

を入力として使用する。

```rust
use argon2::password_hash::{self, SaltString};
use argon2::{Argon2, PasswordHasher, Algorithm, Version, Params};

fn compute_password_hash(password: String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.as_bytes(), &salt)?
    .to_string();
    Ok(password_hash)
}
```

実際に認証機能を実装する際はこの関数の返り値である PHC string format で保存することになるかと思う。
理由は以下のセクションの頭で説明する。

# Argon2 Verifying

PHC string format で保存されていると、その検証は簡単。
`Argon2` を明示的に複数のパラメータを与えて初期化する必要がなくなり、`PasswordVeririer` トレイトの `verify_password` の実装に依存することができる。

```rust
pub trait PasswordVerifier {
    fn verify_password(
        &self,
        password: &[u8],
        hash: &PasswordHash<'_>
    ) -> Result<(), Error>;
}
```

## `PasswordHash` のインスタンスを作成

`PasswordVerifier::verify_password` は期待するパスワードハッシュとして `PasswordHash` を要求する。
これは普通に `PasswordHash::new` で 有効な PHC string format の文字列を渡せばパースされる。

```rust
use argon2::PasswordHash;

let password_hash = PasswordHash::new("valid_phc_format_string")
    .expect("Invalid format.");
```

## Verifying

以上を踏まえて、生のパスワードと期待される PHC string format を引数として受け取り `Result` を返す関数を書いてみる

```rust
use argon2::password_hash::{self, SaltString};
use argon2::{Argon2, PasswordHash, PasswordVerifier};

fn verify_password_hash(
    password: String,
    expected_password_hash: String,
) -> Result<(), password_hash::Error> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.as_str())?;
    Argon2::default().verify_password(password.as_bytes(), &expected_password_hash)
}
```
