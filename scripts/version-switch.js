// scripts/update-symlink.js
import fs from 'fs'
import path from 'path'

const target = process.argv[2]; // 'src-old' 或 'src-new'

try {
  // 检查 src 是否已存在（可能是符号链接或目录）
  if (fs.existsSync('src')) {
    // 如果是符号链接，直接更新
    fs.unlinkSync('src'); // 删除旧链接
  } else if (fs.lstatSync('src').isDirectory()) {
    // 如果是普通目录，报错（避免误删）
    throw new Error('src is a real directory, not a symlink!');
  }

  // 创建新的符号链接
  fs.symlinkSync(target, 'src', 'junction'); // 'junction' 兼容 Windows
  console.log(`Updated symlink: src -> ${target}`);
} catch (err) {
  console.error('Error:', err.message);
  process.exit(1);
}
