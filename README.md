# rust-airtags

## Usage

- .envファイルを以下に必要な項目を入れてrust-airtags直下に作成する

```
APPLE_ID=hogehoge
APPLE_APP_PASSWORD=fugafuga
```

- apple-private-apisのリポジトリをrust-airtagsと同じ階層に置く

- apple-private-apis/omnisette/aos_kit.rsをrust-airtags/fixed_codes/aos_kit.rsに置き換える。

- cargo runを実施する
  - ただし、rustc 1.86.0以上が必要なので、実行したい場合はバージョンを上げる必要があるので注意