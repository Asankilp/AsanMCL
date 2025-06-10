/**
 * Java运行时环境的架构
 */
export enum Architecture {
    X86 = 'X86',
    X86_64 = 'X86_64',
    Arm64 = 'Arm64',
    Unknown = 'Unknown'
}

/**
 * Java运行时环境信息
 */
export interface JreInfo {
    /** JRE 目录的路径 */
    path: string;
    /** Java 版本号 */
    version: string;
    /** 系统架构 */
    arch: Architecture;
    /** 提供者 */
    implementor?: string;
    /** 是否为手动添加 */
    manual?: boolean;
}