use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDEFGHIKLMNPRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

/// 生成随机密码
///
/// 根据指定的长度和字符类型生成随机密码，并打印出密码和密码强度评估。
///
/// # 参数
/// * `length` - 密码的长度
/// * `upper` - 是否包含大写字母
/// * `lower` - 是否包含小写字母
/// * `number` - 是否包含数字
/// * `symbol` - 是否包含符号
///
/// # 返回值
/// * `anyhow::Result` - 如果生成密码成功，返回 Ok(());否则返回错误信息
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    // 根据参数选择性地添加字符集
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"))
    }

    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"))
    }

    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"))
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"))
    }

    // 填充密码到指定长度
    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    // 打乱密码顺序
    password.shuffle(&mut rng);

    // 将密码转换为字符串
    let password = String::from_utf8(password)?;

    Ok(password)
}
