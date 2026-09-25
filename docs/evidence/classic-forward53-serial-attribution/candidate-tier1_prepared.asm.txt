Disassembly of section .text:

00000000001d2d20 <emuella_j2k_tier1::encode_prepared_baseline_code_block>:
  1d2d20:	55                                              	push   %rbp
  1d2d21:	41 57                                           	push   %r15
  1d2d23:	41 56                                           	push   %r14
  1d2d25:	41 55                                           	push   %r13
  1d2d27:	41 54                                           	push   %r12
  1d2d29:	53                                              	push   %rbx
  1d2d2a:	48 81 ec 28 01 00 00                            	sub    $0x128,%rsp
  1d2d31:	49 89 ca                                        	mov    %rcx,%r10
  1d2d34:	48 89 74 24 10                                  	mov    %rsi,0x10(%rsp)
  1d2d39:	48 c1 e9 20                                     	shr    $0x20,%rcx
  1d2d3d:	0f bd f2                                        	bsr    %edx,%esi
  1d2d40:	83 f6 1f                                        	xor    $0x1f,%esi
  1d2d43:	b0 20                                           	mov    $0x20,%al
  1d2d45:	40 28 f0                                        	sub    %sil,%al
  1d2d48:	28 c1                                           	sub    %al,%cl
  1d2d4a:	73 1b                                           	jae    1d2d67 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x47>
  1d2d4c:	c6 07 03                                        	movb   $0x3,(%rdi)
  1d2d4f:	48 8d 05 9c fb e4 ff                            	lea    -0x1b0464(%rip),%rax        # 228f2 <anon.36ada1a7bb6c1d545348b8e8a90a5caf.96.llvm.17933956792295344108+0x41>
  1d2d56:	48 89 47 08                                     	mov    %rax,0x8(%rdi)
  1d2d5a:	48 c7 47 10 32 00 00 00                         	movq   $0x32,0x10(%rdi)
  1d2d62:	e9 0b 16 00 00                                  	jmp    1d4372 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1652>
  1d2d67:	49 c1 ea 28                                     	shr    $0x28,%r10
  1d2d6b:	49 8b 50 10                                     	mov    0x10(%r8),%rdx
  1d2d6f:	0f 57 c0                                        	xorps  %xmm0,%xmm0
  1d2d72:	0f 11 44 24 56                                  	movups %xmm0,0x56(%rsp)
  1d2d77:	0f 11 44 24 46                                  	movups %xmm0,0x46(%rsp)
  1d2d7c:	40 80 f6 1f                                     	xor    $0x1f,%sil
  1d2d80:	40 0f b6 c6                                     	movzbl %sil,%eax
  1d2d84:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d2d87:	fe c0                                           	inc    %al
  1d2d89:	0f b6 c0                                        	movzbl %al,%eax
  1d2d8c:	89 44 24 1c                                     	mov    %eax,0x1c(%rsp)
  1d2d90:	4c 89 44 24 30                                  	mov    %r8,0x30(%rsp)
  1d2d95:	48 89 54 24 28                                  	mov    %rdx,0x28(%rsp)
  1d2d9a:	66 c7 44 24 44 04 00                            	movw   $0x4,0x44(%rsp)
  1d2da1:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d2daa:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d2db2:	48 b8 03 00 2e 00 00 08 08 00                   	movabs $0x80800002e0003,%rax
  1d2dbc:	48 89 44 24 66                                  	mov    %rax,0x66(%rsp)
  1d2dc1:	41 0f b6 c2                                     	movzbl %r10b,%eax
  1d2dc5:	89 44 24 70                                     	mov    %eax,0x70(%rsp)
  1d2dc9:	41 f6 c2 02                                     	test   $0x2,%r10b
  1d2dcd:	4c 89 8c 24 e0 00 00 00                         	mov    %r9,0xe0(%rsp)
  1d2dd5:	4c 89 54 24 08                                  	mov    %r10,0x8(%rsp)
  1d2dda:	89 74 24 74                                     	mov    %esi,0x74(%rsp)
  1d2dde:	48 89 bc 24 08 01 00 00                         	mov    %rdi,0x108(%rsp)
  1d2de6:	48 89 8c 24 20 01 00 00                         	mov    %rcx,0x120(%rsp)
  1d2dee:	48 89 94 24 18 01 00 00                         	mov    %rdx,0x118(%rsp)
  1d2df6:	0f 85 bf 00 00 00                               	jne    1d2ebb <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x19b>
  1d2dfc:	44 89 d0                                        	mov    %r10d,%eax
  1d2dff:	c0 e8 04                                        	shr    $0x4,%al
  1d2e02:	24 01                                           	and    $0x1,%al
  1d2e04:	88 44 24 20                                     	mov    %al,0x20(%rsp)
  1d2e08:	41 f6 c2 08                                     	test   $0x8,%r10b
  1d2e0c:	0f 85 47 0a 00 00                               	jne    1d3859 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xb39>
  1d2e12:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
  1d2e17:	4c 8b 60 40                                     	mov    0x40(%rax),%r12
  1d2e1b:	48 8b 78 38                                     	mov    0x38(%rax),%rdi
  1d2e1f:	48 8b 08                                        	mov    (%rax),%rcx
  1d2e22:	48 89 4c 24 78                                  	mov    %rcx,0x78(%rsp)
  1d2e27:	4c 8b 70 08                                     	mov    0x8(%rax),%r14
  1d2e2b:	4c 8b 40 30                                     	mov    0x30(%rax),%r8
  1d2e2f:	4c 89 84 24 d0 00 00 00                         	mov    %r8,0xd0(%rsp)
  1d2e37:	4b 8d 04 40                                     	lea    (%r8,%r8,2),%rax
  1d2e3b:	48 83 c0 fd                                     	add    $0xfffffffffffffffd,%rax
  1d2e3f:	48 b9 ab aa aa aa aa aa aa aa                   	movabs $0xaaaaaaaaaaaaaaab,%rcx
  1d2e49:	48 89 84 24 b0 00 00 00                         	mov    %rax,0xb0(%rsp)
  1d2e51:	48 f7 e1                                        	mul    %rcx
  1d2e54:	4b 8d 04 40                                     	lea    (%r8,%r8,2),%rax
  1d2e58:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
  1d2e60:	48 d1 ea                                        	shr    $1,%rdx
  1d2e63:	8d 42 01                                        	lea    0x1(%rdx),%eax
  1d2e66:	83 e0 07                                        	and    $0x7,%eax
  1d2e69:	83 e2 07                                        	and    $0x7,%edx
  1d2e6c:	48 89 94 24 a8 00 00 00                         	mov    %rdx,0xa8(%rsp)
  1d2e74:	48 f7 d8                                        	neg    %rax
  1d2e77:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
  1d2e7f:	48 89 bc 24 80 00 00 00                         	mov    %rdi,0x80(%rsp)
  1d2e87:	48 8d 47 01                                     	lea    0x1(%rdi),%rax
  1d2e8b:	48 89 84 24 10 01 00 00                         	mov    %rax,0x110(%rsp)
  1d2e93:	45 31 ff                                        	xor    %r15d,%r15d
  1d2e96:	48 8d 84 24 e8 00 00 00                         	lea    0xe8(%rsp),%rax
  1d2e9e:	48 89 84 24 d8 00 00 00                         	mov    %rax,0xd8(%rsp)
  1d2ea6:	4c 89 b4 24 90 00 00 00                         	mov    %r14,0x90(%rsp)
  1d2eae:	4c 89 a4 24 88 00 00 00                         	mov    %r12,0x88(%rsp)
  1d2eb6:	e9 e5 00 00 00                                  	jmp    1d2fa0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x280>
  1d2ebb:	41 f6 c2 08                                     	test   $0x8,%r10b
  1d2ebf:	0f 85 84 0e 00 00                               	jne    1d3d49 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1029>
  1d2ec5:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
  1d2eca:	48 8b 48 40                                     	mov    0x40(%rax),%rcx
  1d2ece:	48 89 8c 24 90 00 00 00                         	mov    %rcx,0x90(%rsp)
  1d2ed6:	48 8b 78 38                                     	mov    0x38(%rax),%rdi
  1d2eda:	48 89 bc 24 88 00 00 00                         	mov    %rdi,0x88(%rsp)
  1d2ee2:	48 8b 08                                        	mov    (%rax),%rcx
  1d2ee5:	48 89 8c 24 80 00 00 00                         	mov    %rcx,0x80(%rsp)
  1d2eed:	4c 8b 78 08                                     	mov    0x8(%rax),%r15
  1d2ef1:	48 8b 40 30                                     	mov    0x30(%rax),%rax
  1d2ef5:	48 89 44 24 78                                  	mov    %rax,0x78(%rsp)
  1d2efa:	48 8d 0c 40                                     	lea    (%rax,%rax,2),%rcx
  1d2efe:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  1d2f03:	45 89 d4                                        	mov    %r10d,%r12d
  1d2f06:	41 c0 ec 04                                     	shr    $0x4,%r12b
  1d2f0a:	41 80 e4 01                                     	and    $0x1,%r12b
  1d2f0e:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
  1d2f12:	48 83 c0 fd                                     	add    $0xfffffffffffffffd,%rax
  1d2f16:	48 b9 ab aa aa aa aa aa aa aa                   	movabs $0xaaaaaaaaaaaaaaab,%rcx
  1d2f20:	48 89 84 24 b0 00 00 00                         	mov    %rax,0xb0(%rsp)
  1d2f28:	48 f7 e1                                        	mul    %rcx
  1d2f2b:	48 d1 ea                                        	shr    $1,%rdx
  1d2f2e:	8d 42 01                                        	lea    0x1(%rdx),%eax
  1d2f31:	83 e0 07                                        	and    $0x7,%eax
  1d2f34:	83 e2 07                                        	and    $0x7,%edx
  1d2f37:	48 89 94 24 a8 00 00 00                         	mov    %rdx,0xa8(%rsp)
  1d2f3f:	48 f7 d8                                        	neg    %rax
  1d2f42:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
  1d2f4a:	48 8d 47 01                                     	lea    0x1(%rdi),%rax
  1d2f4e:	48 89 84 24 d0 00 00 00                         	mov    %rax,0xd0(%rsp)
  1d2f56:	45 31 f6                                        	xor    %r14d,%r14d
  1d2f59:	48 8d 84 24 f0 00 00 00                         	lea    0xf0(%rsp),%rax
  1d2f61:	48 89 84 24 d8 00 00 00                         	mov    %rax,0xd8(%rsp)
  1d2f69:	4c 89 bc 24 98 00 00 00                         	mov    %r15,0x98(%rsp)
  1d2f71:	e9 9a 04 00 00                                  	jmp    1d3410 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6f0>
  1d2f76:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d2f7f:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d2f87:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d2f90:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d2f96:	8b 74 24 74                                     	mov    0x74(%rsp),%esi
  1d2f9a:	0f 84 da 12 00 00                               	je     1d427a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x155a>
  1d2fa0:	41 0f b7 c7                                     	movzwl %r15w,%eax
  1d2fa4:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d2faa:	c1 e8 11                                        	shr    $0x11,%eax
  1d2fad:	8d 0c 40                                        	lea    (%rax,%rax,2),%ecx
  1d2fb0:	31 d2                                           	xor    %edx,%edx
  1d2fb2:	44 89 fb                                        	mov    %r15d,%ebx
  1d2fb5:	66 29 cb                                        	sub    %cx,%bx
  1d2fb8:	0f 95 c2                                        	setne  %dl
  1d2fbb:	01 c2                                           	add    %eax,%edx
  1d2fbd:	b0 03                                           	mov    $0x3,%al
  1d2fbf:	89 f1                                           	mov    %esi,%ecx
  1d2fc1:	28 d1                                           	sub    %dl,%cl
  1d2fc3:	0f 82 a0 12 00 00                               	jb     1d4269 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1549>
  1d2fc9:	44 89 fd                                        	mov    %r15d,%ebp
  1d2fcc:	66 85 db                                        	test   %bx,%bx
  1d2fcf:	0f 95 c2                                        	setne  %dl
  1d2fd2:	66 41 83 ff 0a                                  	cmp    $0xa,%r15w
  1d2fd7:	41 0f 93 c5                                     	setae  %r13b
  1d2fdb:	45 20 d5                                        	and    %r10b,%r13b
  1d2fde:	41 20 d5                                        	and    %dl,%r13b
  1d2fe1:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  1d2fe6:	88 4a 48                                        	mov    %cl,0x48(%rdx)
  1d2fe9:	66 85 db                                        	test   %bx,%bx
  1d2fec:	74 32                                           	je     1d3020 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x300>
  1d2fee:	0f b7 c3                                        	movzwl %bx,%eax
  1d2ff1:	83 f8 01                                        	cmp    $0x1,%eax
  1d2ff4:	0f 85 86 01 00 00                               	jne    1d3180 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x460>
  1d2ffa:	45 84 ed                                        	test   %r13b,%r13b
  1d2ffd:	0f 84 93 01 00 00                               	je     1d3196 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x476>
  1d3003:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d3008:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d300d:	e8 6e 8c fe ff                                  	call   1bbc80 <emuella_j2k_tier1::significance_propagation_pass_encode_raw::<false>>
  1d3012:	e9 9f 01 00 00                                  	jmp    1d31b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x496>
  1d3017:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d3020:	45 84 ed                                        	test   %r13b,%r13b
  1d3023:	4c 8b bc 24 80 00 00 00                         	mov    0x80(%rsp),%r15
  1d302b:	0f 85 85 12 00 00                               	jne    1d42b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1596>
  1d3031:	48 8d bc 24 b8 00 00 00                         	lea    0xb8(%rsp),%rdi
  1d3039:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1d303e:	48 8d 54 24 28                                  	lea    0x28(%rsp),%rdx
  1d3043:	e8 d8 e2 fd ff                                  	call   1b1320 <emuella_j2k_tier1::cleanup_pass_encode::<false>>
  1d3048:	0f b6 84 24 b8 00 00 00                         	movzbl 0xb8(%rsp),%eax
  1d3050:	3c ff                                           	cmp    $0xff,%al
  1d3052:	0f 85 6f 12 00 00                               	jne    1d42c7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x15a7>
  1d3058:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d305d:	41 f6 c2 20                                     	test   $0x20,%r10b
  1d3061:	74 4f                                           	je     1d30b2 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x392>
  1d3063:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d3068:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d306d:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d3072:	e8 89 85 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d3077:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d307c:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3081:	31 d2                                           	xor    %edx,%edx
  1d3083:	e8 78 85 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d3088:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d308d:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3092:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d3097:	e8 64 85 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d309c:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d30a1:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d30a6:	31 d2                                           	xor    %edx,%edx
  1d30a8:	e8 53 85 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d30ad:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d30b2:	4d 85 e4                                        	test   %r12,%r12
  1d30b5:	44 0f b6 44 24 20                               	movzbl 0x20(%rsp),%r8d
  1d30bb:	4c 8b 4c 24 78                                  	mov    0x78(%rsp),%r9
  1d30c0:	4c 8b 9c 24 d0 00 00 00                         	mov    0xd0(%rsp),%r11
  1d30c8:	0f 84 f3 00 00 00                               	je     1d31c1 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4a1>
  1d30ce:	4d 85 db                                        	test   %r11,%r11
  1d30d1:	0f 84 d1 02 00 00                               	je     1d33a8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x688>
  1d30d7:	31 c0                                           	xor    %eax,%eax
  1d30d9:	eb 0e                                           	jmp    1d30e9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x3c9>
  1d30db:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
  1d30e0:	4c 39 e0                                        	cmp    %r12,%rax
  1d30e3:	0f 84 d8 00 00 00                               	je     1d31c1 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4a1>
  1d30e9:	48 ff c0                                        	inc    %rax
  1d30ec:	48 89 c7                                        	mov    %rax,%rdi
  1d30ef:	49 0f af ff                                     	imul   %r15,%rdi
  1d30f3:	48 ff c7                                        	inc    %rdi
  1d30f6:	48 89 fe                                        	mov    %rdi,%rsi
  1d30f9:	4c 01 de                                        	add    %r11,%rsi
  1d30fc:	0f 82 98 12 00 00                               	jb     1d439a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x167a>
  1d3102:	4c 39 f6                                        	cmp    %r14,%rsi
  1d3105:	0f 87 8f 12 00 00                               	ja     1d439a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x167a>
  1d310b:	48 8d 0c 7f                                     	lea    (%rdi,%rdi,2),%rcx
  1d310f:	4c 01 c9                                        	add    %r9,%rcx
  1d3112:	48 89 ca                                        	mov    %rcx,%rdx
  1d3115:	83 bc 24 a8 00 00 00 07                         	cmpl   $0x7,0xa8(%rsp)
  1d311d:	74 1e                                           	je     1d313d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x41d>
  1d311f:	48 8b b4 24 98 00 00 00                         	mov    0x98(%rsp),%rsi
  1d3127:	48 89 ca                                        	mov    %rcx,%rdx
  1d312a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d3130:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d3134:	48 83 c2 03                                     	add    $0x3,%rdx
  1d3138:	48 ff c6                                        	inc    %rsi
  1d313b:	75 f3                                           	jne    1d3130 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x410>
  1d313d:	48 83 bc 24 b0 00 00 00 15                      	cmpq   $0x15,0xb0(%rsp)
  1d3146:	72 98                                           	jb     1d30e0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x3c0>
  1d3148:	48 03 8c 24 a0 00 00 00                         	add    0xa0(%rsp),%rcx
  1d3150:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d3154:	c6 42 04 00                                     	movb   $0x0,0x4(%rdx)
  1d3158:	c6 42 07 00                                     	movb   $0x0,0x7(%rdx)
  1d315c:	c6 42 0a 00                                     	movb   $0x0,0xa(%rdx)
  1d3160:	c6 42 0d 00                                     	movb   $0x0,0xd(%rdx)
  1d3164:	c6 42 10 00                                     	movb   $0x0,0x10(%rdx)
  1d3168:	c6 42 13 00                                     	movb   $0x0,0x13(%rdx)
  1d316c:	c6 42 16 00                                     	movb   $0x0,0x16(%rdx)
  1d3170:	48 83 c2 18                                     	add    $0x18,%rdx
  1d3174:	48 39 ca                                        	cmp    %rcx,%rdx
  1d3177:	75 d7                                           	jne    1d3150 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x430>
  1d3179:	e9 62 ff ff ff                                  	jmp    1d30e0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x3c0>
  1d317e:	66 90                                           	xchg   %ax,%ax
  1d3180:	45 84 ed                                        	test   %r13b,%r13b
  1d3183:	74 22                                           	je     1d31a7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x487>
  1d3185:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d318a:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d318f:	e8 2c 12 00 00                                  	call   1d43c0 <emuella_j2k_tier1::magnitude_refinement_pass_encode_raw>
  1d3194:	eb 20                                           	jmp    1d31b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x496>
  1d3196:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d319b:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d31a0:	e8 fb 73 fe ff                                  	call   1ba5a0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>>
  1d31a5:	eb 0f                                           	jmp    1d31b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x496>
  1d31a7:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d31ac:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d31b1:	e8 6a 53 fe ff                                  	call   1b8520 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>>
  1d31b6:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d31bb:	44 0f b6 44 24 20                               	movzbl 0x20(%rsp),%r8d
  1d31c1:	44 8d 7d 01                                     	lea    0x1(%rbp),%r15d
  1d31c5:	41 f6 c2 04                                     	test   $0x4,%r10b
  1d31c9:	75 35                                           	jne    1d3200 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4e0>
  1d31cb:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d31d1:	74 2d                                           	je     1d3200 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4e0>
  1d31d3:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d31d7:	0f 84 b3 fd ff ff                               	je     1d2f90 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x270>
  1d31dd:	66 83 fd 09                                     	cmp    $0x9,%bp
  1d31e1:	74 1d                                           	je     1d3200 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4e0>
  1d31e3:	0f 96 c0                                        	setbe  %al
  1d31e6:	66 83 fb 01                                     	cmp    $0x1,%bx
  1d31ea:	0f 94 c1                                        	sete   %cl
  1d31ed:	08 c1                                           	or     %al,%cl
  1d31ef:	0f 85 9b fd ff ff                               	jne    1d2f90 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x270>
  1d31f5:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  1d3200:	45 84 ed                                        	test   %r13b,%r13b
  1d3203:	74 3b                                           	je     1d3240 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x520>
  1d3205:	0f b6 44 24 6c                                  	movzbl 0x6c(%rsp),%eax
  1d320a:	0f b6 54 24 6b                                  	movzbl 0x6b(%rsp),%edx
  1d320f:	48 8b 4c 24 28                                  	mov    0x28(%rsp),%rcx
  1d3214:	48 8b 5c 24 30                                  	mov    0x30(%rsp),%rbx
  1d3219:	4c 8b 6b 10                                     	mov    0x10(%rbx),%r13
  1d321d:	49 39 cd                                        	cmp    %rcx,%r13
  1d3220:	76 36                                           	jbe    1d3258 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x538>
  1d3222:	38 d0                                           	cmp    %dl,%al
  1d3224:	72 3a                                           	jb     1d3260 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x540>
  1d3226:	48 8b 53 08                                     	mov    0x8(%rbx),%rdx
  1d322a:	48 01 ca                                        	add    %rcx,%rdx
  1d322d:	4c 01 ea                                        	add    %r13,%rdx
  1d3230:	48 f7 d1                                        	not    %rcx
  1d3233:	80 3c 11 ff                                     	cmpb   $0xff,(%rcx,%rdx,1)
  1d3237:	0f 85 c3 00 00 00                               	jne    1d3300 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5e0>
  1d323d:	eb 21                                           	jmp    1d3260 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x540>
  1d323f:	90                                              	nop
  1d3240:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d3245:	8b 74 24 70                                     	mov    0x70(%rsp),%esi
  1d3249:	e8 f2 ef ff ff                                  	call   1d2240 <emuella_j2k_tier1::terminate_arithmetic_segment>
  1d324e:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3253:	e9 a8 00 00 00                                  	jmp    1d3300 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5e0>
  1d3258:	38 d0                                           	cmp    %dl,%al
  1d325a:	0f 83 a0 00 00 00                               	jae    1d3300 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5e0>
  1d3260:	44 0f b6 74 24 6a                               	movzbl 0x6a(%rsp),%r14d
  1d3266:	84 c0                                           	test   %al,%al
  1d3268:	74 5c                                           	je     1d32c6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5a6>
  1d326a:	89 c2                                           	mov    %eax,%edx
  1d326c:	80 e2 03                                        	and    $0x3,%dl
  1d326f:	3c 04                                           	cmp    $0x4,%al
  1d3271:	72 36                                           	jb     1d32a9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x589>
  1d3273:	89 c6                                           	mov    %eax,%esi
  1d3275:	40 80 e6 fc                                     	and    $0xfc,%sil
  1d3279:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d3280:	8d 48 06                                        	lea    0x6(%rax),%ecx
  1d3283:	80 e1 07                                        	and    $0x7,%cl
  1d3286:	44 89 c7                                        	mov    %r8d,%edi
  1d3289:	40 d2 e7                                        	shl    %cl,%dil
  1d328c:	44 08 f7                                        	or     %r14b,%dil
  1d328f:	04 fc                                           	add    $0xfc,%al
  1d3291:	89 c1                                           	mov    %eax,%ecx
  1d3293:	80 e1 07                                        	and    $0x7,%cl
  1d3296:	45 89 c6                                        	mov    %r8d,%r14d
  1d3299:	41 d2 e6                                        	shl    %cl,%r14b
  1d329c:	41 08 fe                                        	or     %dil,%r14b
  1d329f:	40 80 c6 fc                                     	add    $0xfc,%sil
  1d32a3:	75 db                                           	jne    1d3280 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x560>
  1d32a5:	84 d2                                           	test   %dl,%dl
  1d32a7:	74 1d                                           	je     1d32c6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5a6>
  1d32a9:	fe c8                                           	dec    %al
  1d32ab:	31 f6                                           	xor    %esi,%esi
  1d32ad:	0f 1f 00                                        	nopl   (%rax)
  1d32b0:	89 c1                                           	mov    %eax,%ecx
  1d32b2:	80 e1 07                                        	and    $0x7,%cl
  1d32b5:	89 f7                                           	mov    %esi,%edi
  1d32b7:	40 d2 e7                                        	shl    %cl,%dil
  1d32ba:	41 08 fe                                        	or     %dil,%r14b
  1d32bd:	44 30 c6                                        	xor    %r8b,%sil
  1d32c0:	fe c8                                           	dec    %al
  1d32c2:	fe ca                                           	dec    %dl
  1d32c4:	75 ea                                           	jne    1d32b0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x590>
  1d32c6:	4c 3b 2b                                        	cmp    (%rbx),%r13
  1d32c9:	75 0e                                           	jne    1d32d9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5b9>
  1d32cb:	48 89 df                                        	mov    %rbx,%rdi
  1d32ce:	ff 15 6c 1b 0a 00                               	call   *0xa1b6c(%rip)        # 274e40 <_DYNAMIC+0x290>
  1d32d4:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d32d9:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1d32dd:	46 88 34 28                                     	mov    %r14b,(%rax,%r13,1)
  1d32e1:	49 ff c5                                        	inc    %r13
  1d32e4:	4c 89 6b 10                                     	mov    %r13,0x10(%rbx)
  1d32e8:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1d32ec:	0f 94 c0                                        	sete   %al
  1d32ef:	b1 08                                           	mov    $0x8,%cl
  1d32f1:	28 c1                                           	sub    %al,%cl
  1d32f3:	88 4c 24 6b                                     	mov    %cl,0x6b(%rsp)
  1d32f7:	88 4c 24 6c                                     	mov    %cl,0x6c(%rsp)
  1d32fb:	c6 44 24 6a 00                                  	movb   $0x0,0x6a(%rsp)
  1d3300:	4c 8b a4 24 e0 00 00 00                         	mov    0xe0(%rsp),%r12
  1d3308:	4d 85 e4                                        	test   %r12,%r12
  1d330b:	74 2e                                           	je     1d333b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x61b>
  1d330d:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d3312:	48 8b 58 10                                     	mov    0x10(%rax),%rbx
  1d3316:	48 2b 5c 24 28                                  	sub    0x28(%rsp),%rbx
  1d331b:	4d 8b 74 24 10                                  	mov    0x10(%r12),%r14
  1d3320:	4d 3b 34 24                                     	cmp    (%r12),%r14
  1d3324:	0f 84 ac 00 00 00                               	je     1d33d6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6b6>
  1d332a:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
  1d332f:	4a 89 1c f0                                     	mov    %rbx,(%rax,%r14,8)
  1d3333:	49 ff c6                                        	inc    %r14
  1d3336:	4d 89 74 24 10                                  	mov    %r14,0x10(%r12)
  1d333b:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d3341:	4c 8b b4 24 90 00 00 00                         	mov    0x90(%rsp),%r14
  1d3349:	4c 8b a4 24 88 00 00 00                         	mov    0x88(%rsp),%r12
  1d3351:	0f 83 39 fc ff ff                               	jae    1d2f90 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x270>
  1d3357:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d335c:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d3360:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1d3365:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d3369:	0f 84 07 fc ff ff                               	je     1d2f76 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x256>
  1d336f:	66 83 fd 09                                     	cmp    $0x9,%bp
  1d3373:	0f 82 fd fb ff ff                               	jb     1d2f76 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x256>
  1d3379:	41 0f b7 c7                                     	movzwl %r15w,%eax
  1d337d:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d3383:	c1 e8 11                                        	shr    $0x11,%eax
  1d3386:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d3389:	44 89 f9                                        	mov    %r15d,%ecx
  1d338c:	29 c1                                           	sub    %eax,%ecx
  1d338e:	66 85 c9                                        	test   %cx,%cx
  1d3391:	0f 84 df fb ff ff                               	je     1d2f76 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x256>
  1d3397:	66 c7 44 24 6a 00 08                            	movw   $0x800,0x6a(%rsp)
  1d339e:	c6 44 24 6c 08                                  	movb   $0x8,0x6c(%rsp)
  1d33a3:	e9 e8 fb ff ff                                  	jmp    1d2f90 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x270>
  1d33a8:	48 8b bc 24 10 01 00 00                         	mov    0x110(%rsp),%rdi
  1d33b0:	4c 89 e0                                        	mov    %r12,%rax
  1d33b3:	66 66 66 66 2e 0f 1f 84 00 00 00 00 00          	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  1d33c0:	4c 39 f7                                        	cmp    %r14,%rdi
  1d33c3:	0f 87 ce 0f 00 00                               	ja     1d4397 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1677>
  1d33c9:	4c 01 ff                                        	add    %r15,%rdi
  1d33cc:	48 ff c8                                        	dec    %rax
  1d33cf:	75 ef                                           	jne    1d33c0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6a0>
  1d33d1:	e9 eb fd ff ff                                  	jmp    1d31c1 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4a1>
  1d33d6:	4c 89 e7                                        	mov    %r12,%rdi
  1d33d9:	ff 15 79 1e 0a 00                               	call   *0xa1e79(%rip)        # 275258 <_DYNAMIC+0x6a8>
  1d33df:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d33e4:	e9 41 ff ff ff                                  	jmp    1d332a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x60a>
  1d33e9:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d33f2:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d33fa:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d3400:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d3406:	8b 74 24 74                                     	mov    0x74(%rsp),%esi
  1d340a:	0f 84 6a 0e 00 00                               	je     1d427a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x155a>
  1d3410:	41 0f b7 c6                                     	movzwl %r14w,%eax
  1d3414:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d341a:	c1 e8 11                                        	shr    $0x11,%eax
  1d341d:	8d 0c 40                                        	lea    (%rax,%rax,2),%ecx
  1d3420:	31 d2                                           	xor    %edx,%edx
  1d3422:	44 89 f5                                        	mov    %r14d,%ebp
  1d3425:	66 29 cd                                        	sub    %cx,%bp
  1d3428:	0f 95 c2                                        	setne  %dl
  1d342b:	01 c2                                           	add    %eax,%edx
  1d342d:	b0 03                                           	mov    $0x3,%al
  1d342f:	89 f1                                           	mov    %esi,%ecx
  1d3431:	28 d1                                           	sub    %dl,%cl
  1d3433:	0f 82 30 0e 00 00                               	jb     1d4269 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1549>
  1d3439:	44 89 f3                                        	mov    %r14d,%ebx
  1d343c:	66 85 ed                                        	test   %bp,%bp
  1d343f:	0f 95 c2                                        	setne  %dl
  1d3442:	66 41 83 fe 0a                                  	cmp    $0xa,%r14w
  1d3447:	41 0f 93 c5                                     	setae  %r13b
  1d344b:	45 20 d5                                        	and    %r10b,%r13b
  1d344e:	41 20 d5                                        	and    %dl,%r13b
  1d3451:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  1d3456:	88 4a 48                                        	mov    %cl,0x48(%rdx)
  1d3459:	66 85 ed                                        	test   %bp,%bp
  1d345c:	74 32                                           	je     1d3490 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x770>
  1d345e:	0f b7 c5                                        	movzwl %bp,%eax
  1d3461:	83 f8 01                                        	cmp    $0x1,%eax
  1d3464:	0f 85 86 01 00 00                               	jne    1d35f0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x8d0>
  1d346a:	45 84 ed                                        	test   %r13b,%r13b
  1d346d:	0f 84 93 01 00 00                               	je     1d3606 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x8e6>
  1d3473:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d3478:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d347d:	e8 fe 87 fe ff                                  	call   1bbc80 <emuella_j2k_tier1::significance_propagation_pass_encode_raw::<false>>
  1d3482:	e9 9f 01 00 00                                  	jmp    1d3626 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x906>
  1d3487:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d3490:	45 84 ed                                        	test   %r13b,%r13b
  1d3493:	4c 8b b4 24 90 00 00 00                         	mov    0x90(%rsp),%r14
  1d349b:	0f 85 15 0e 00 00                               	jne    1d42b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1596>
  1d34a1:	48 8d bc 24 b8 00 00 00                         	lea    0xb8(%rsp),%rdi
  1d34a9:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1d34ae:	48 8d 54 24 28                                  	lea    0x28(%rsp),%rdx
  1d34b3:	e8 68 de fd ff                                  	call   1b1320 <emuella_j2k_tier1::cleanup_pass_encode::<false>>
  1d34b8:	0f b6 84 24 b8 00 00 00                         	movzbl 0xb8(%rsp),%eax
  1d34c0:	3c ff                                           	cmp    $0xff,%al
  1d34c2:	0f 85 1d 0e 00 00                               	jne    1d42e5 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x15c5>
  1d34c8:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d34cd:	41 f6 c2 20                                     	test   $0x20,%r10b
  1d34d1:	74 4f                                           	je     1d3522 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x802>
  1d34d3:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d34d8:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d34dd:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d34e2:	e8 19 81 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d34e7:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d34ec:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d34f1:	31 d2                                           	xor    %edx,%edx
  1d34f3:	e8 08 81 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d34f8:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d34fd:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3502:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d3507:	e8 f4 80 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d350c:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d3511:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3516:	31 d2                                           	xor    %edx,%edx
  1d3518:	e8 e3 80 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d351d:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3522:	4d 85 f6                                        	test   %r14,%r14
  1d3525:	4c 8b 84 24 88 00 00 00                         	mov    0x88(%rsp),%r8
  1d352d:	4c 8b 8c 24 80 00 00 00                         	mov    0x80(%rsp),%r9
  1d3535:	4c 8b 5c 24 78                                  	mov    0x78(%rsp),%r11
  1d353a:	0f 84 eb 00 00 00                               	je     1d362b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x90b>
  1d3540:	4d 85 db                                        	test   %r11,%r11
  1d3543:	0f 84 d5 02 00 00                               	je     1d381e <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xafe>
  1d3549:	31 c0                                           	xor    %eax,%eax
  1d354b:	eb 0c                                           	jmp    1d3559 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x839>
  1d354d:	0f 1f 00                                        	nopl   (%rax)
  1d3550:	4c 39 f0                                        	cmp    %r14,%rax
  1d3553:	0f 84 d2 00 00 00                               	je     1d362b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x90b>
  1d3559:	48 ff c0                                        	inc    %rax
  1d355c:	48 89 c7                                        	mov    %rax,%rdi
  1d355f:	49 0f af f8                                     	imul   %r8,%rdi
  1d3563:	48 ff c7                                        	inc    %rdi
  1d3566:	48 89 fe                                        	mov    %rdi,%rsi
  1d3569:	4c 01 de                                        	add    %r11,%rsi
  1d356c:	0f 82 15 0e 00 00                               	jb     1d4387 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1667>
  1d3572:	4c 39 fe                                        	cmp    %r15,%rsi
  1d3575:	0f 87 0c 0e 00 00                               	ja     1d4387 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1667>
  1d357b:	48 8d 0c 7f                                     	lea    (%rdi,%rdi,2),%rcx
  1d357f:	4c 01 c9                                        	add    %r9,%rcx
  1d3582:	48 89 ca                                        	mov    %rcx,%rdx
  1d3585:	83 bc 24 a8 00 00 00 07                         	cmpl   $0x7,0xa8(%rsp)
  1d358d:	74 1e                                           	je     1d35ad <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x88d>
  1d358f:	48 8b b4 24 a0 00 00 00                         	mov    0xa0(%rsp),%rsi
  1d3597:	48 89 ca                                        	mov    %rcx,%rdx
  1d359a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d35a0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d35a4:	48 83 c2 03                                     	add    $0x3,%rdx
  1d35a8:	48 ff c6                                        	inc    %rsi
  1d35ab:	75 f3                                           	jne    1d35a0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x880>
  1d35ad:	48 83 bc 24 b0 00 00 00 15                      	cmpq   $0x15,0xb0(%rsp)
  1d35b6:	72 98                                           	jb     1d3550 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x830>
  1d35b8:	48 03 4c 24 20                                  	add    0x20(%rsp),%rcx
  1d35bd:	0f 1f 00                                        	nopl   (%rax)
  1d35c0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d35c4:	c6 42 04 00                                     	movb   $0x0,0x4(%rdx)
  1d35c8:	c6 42 07 00                                     	movb   $0x0,0x7(%rdx)
  1d35cc:	c6 42 0a 00                                     	movb   $0x0,0xa(%rdx)
  1d35d0:	c6 42 0d 00                                     	movb   $0x0,0xd(%rdx)
  1d35d4:	c6 42 10 00                                     	movb   $0x0,0x10(%rdx)
  1d35d8:	c6 42 13 00                                     	movb   $0x0,0x13(%rdx)
  1d35dc:	c6 42 16 00                                     	movb   $0x0,0x16(%rdx)
  1d35e0:	48 83 c2 18                                     	add    $0x18,%rdx
  1d35e4:	48 39 ca                                        	cmp    %rcx,%rdx
  1d35e7:	75 d7                                           	jne    1d35c0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x8a0>
  1d35e9:	e9 62 ff ff ff                                  	jmp    1d3550 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x830>
  1d35ee:	66 90                                           	xchg   %ax,%ax
  1d35f0:	45 84 ed                                        	test   %r13b,%r13b
  1d35f3:	74 22                                           	je     1d3617 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x8f7>
  1d35f5:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d35fa:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d35ff:	e8 bc 0d 00 00                                  	call   1d43c0 <emuella_j2k_tier1::magnitude_refinement_pass_encode_raw>
  1d3604:	eb 20                                           	jmp    1d3626 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x906>
  1d3606:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d360b:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d3610:	e8 8b 6f fe ff                                  	call   1ba5a0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>>
  1d3615:	eb 0f                                           	jmp    1d3626 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x906>
  1d3617:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d361c:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d3621:	e8 fa 4e fe ff                                  	call   1b8520 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>>
  1d3626:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d362b:	44 8d 73 01                                     	lea    0x1(%rbx),%r14d
  1d362f:	c6 44 24 44 04                                  	movb   $0x4,0x44(%rsp)
  1d3634:	48 8d 44 24 45                                  	lea    0x45(%rsp),%rax
  1d3639:	0f 57 c0                                        	xorps  %xmm0,%xmm0
  1d363c:	0f 11 40 10                                     	movups %xmm0,0x10(%rax)
  1d3640:	0f 11 00                                        	movups %xmm0,(%rax)
  1d3643:	c6 40 20 00                                     	movb   $0x0,0x20(%rax)
  1d3647:	c7 44 24 66 03 00 2e 00                         	movl   $0x2e0003,0x66(%rsp)
  1d364f:	41 f6 c2 04                                     	test   $0x4,%r10b
  1d3653:	75 2b                                           	jne    1d3680 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x960>
  1d3655:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d365b:	74 23                                           	je     1d3680 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x960>
  1d365d:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d3661:	0f 84 99 fd ff ff                               	je     1d3400 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6e0>
  1d3667:	66 83 fb 09                                     	cmp    $0x9,%bx
  1d366b:	74 13                                           	je     1d3680 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x960>
  1d366d:	0f 96 c0                                        	setbe  %al
  1d3670:	66 83 fd 01                                     	cmp    $0x1,%bp
  1d3674:	0f 94 c1                                        	sete   %cl
  1d3677:	08 c1                                           	or     %al,%cl
  1d3679:	0f 85 81 fd ff ff                               	jne    1d3400 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6e0>
  1d367f:	90                                              	nop
  1d3680:	45 84 ed                                        	test   %r13b,%r13b
  1d3683:	74 3b                                           	je     1d36c0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9a0>
  1d3685:	0f b6 44 24 6c                                  	movzbl 0x6c(%rsp),%eax
  1d368a:	0f b6 54 24 6b                                  	movzbl 0x6b(%rsp),%edx
  1d368f:	48 8b 4c 24 28                                  	mov    0x28(%rsp),%rcx
  1d3694:	48 8b 6c 24 30                                  	mov    0x30(%rsp),%rbp
  1d3699:	4c 8b 6d 10                                     	mov    0x10(%rbp),%r13
  1d369d:	49 39 cd                                        	cmp    %rcx,%r13
  1d36a0:	76 36                                           	jbe    1d36d8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9b8>
  1d36a2:	38 d0                                           	cmp    %dl,%al
  1d36a4:	72 3a                                           	jb     1d36e0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9c0>
  1d36a6:	48 8b 55 08                                     	mov    0x8(%rbp),%rdx
  1d36aa:	48 01 ca                                        	add    %rcx,%rdx
  1d36ad:	4c 01 ea                                        	add    %r13,%rdx
  1d36b0:	48 f7 d1                                        	not    %rcx
  1d36b3:	80 3c 11 ff                                     	cmpb   $0xff,(%rcx,%rdx,1)
  1d36b7:	0f 85 c4 00 00 00                               	jne    1d3781 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa61>
  1d36bd:	eb 21                                           	jmp    1d36e0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9c0>
  1d36bf:	90                                              	nop
  1d36c0:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d36c5:	8b 74 24 70                                     	mov    0x70(%rsp),%esi
  1d36c9:	e8 72 eb ff ff                                  	call   1d2240 <emuella_j2k_tier1::terminate_arithmetic_segment>
  1d36ce:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d36d3:	e9 a9 00 00 00                                  	jmp    1d3781 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa61>
  1d36d8:	38 d0                                           	cmp    %dl,%al
  1d36da:	0f 83 a1 00 00 00                               	jae    1d3781 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa61>
  1d36e0:	44 0f b6 7c 24 6a                               	movzbl 0x6a(%rsp),%r15d
  1d36e6:	84 c0                                           	test   %al,%al
  1d36e8:	74 5c                                           	je     1d3746 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa26>
  1d36ea:	89 c2                                           	mov    %eax,%edx
  1d36ec:	80 e2 03                                        	and    $0x3,%dl
  1d36ef:	3c 04                                           	cmp    $0x4,%al
  1d36f1:	72 36                                           	jb     1d3729 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa09>
  1d36f3:	89 c6                                           	mov    %eax,%esi
  1d36f5:	40 80 e6 fc                                     	and    $0xfc,%sil
  1d36f9:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d3700:	8d 48 06                                        	lea    0x6(%rax),%ecx
  1d3703:	80 e1 07                                        	and    $0x7,%cl
  1d3706:	44 89 e7                                        	mov    %r12d,%edi
  1d3709:	40 d2 e7                                        	shl    %cl,%dil
  1d370c:	44 08 ff                                        	or     %r15b,%dil
  1d370f:	04 fc                                           	add    $0xfc,%al
  1d3711:	89 c1                                           	mov    %eax,%ecx
  1d3713:	80 e1 07                                        	and    $0x7,%cl
  1d3716:	45 89 e7                                        	mov    %r12d,%r15d
  1d3719:	41 d2 e7                                        	shl    %cl,%r15b
  1d371c:	41 08 ff                                        	or     %dil,%r15b
  1d371f:	40 80 c6 fc                                     	add    $0xfc,%sil
  1d3723:	75 db                                           	jne    1d3700 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9e0>
  1d3725:	84 d2                                           	test   %dl,%dl
  1d3727:	74 1d                                           	je     1d3746 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa26>
  1d3729:	fe c8                                           	dec    %al
  1d372b:	31 f6                                           	xor    %esi,%esi
  1d372d:	0f 1f 00                                        	nopl   (%rax)
  1d3730:	89 c1                                           	mov    %eax,%ecx
  1d3732:	80 e1 07                                        	and    $0x7,%cl
  1d3735:	89 f7                                           	mov    %esi,%edi
  1d3737:	40 d2 e7                                        	shl    %cl,%dil
  1d373a:	41 08 ff                                        	or     %dil,%r15b
  1d373d:	44 30 e6                                        	xor    %r12b,%sil
  1d3740:	fe c8                                           	dec    %al
  1d3742:	fe ca                                           	dec    %dl
  1d3744:	75 ea                                           	jne    1d3730 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa10>
  1d3746:	4c 3b 6d 00                                     	cmp    0x0(%rbp),%r13
  1d374a:	75 0e                                           	jne    1d375a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa3a>
  1d374c:	48 89 ef                                        	mov    %rbp,%rdi
  1d374f:	ff 15 eb 16 0a 00                               	call   *0xa16eb(%rip)        # 274e40 <_DYNAMIC+0x290>
  1d3755:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d375a:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1d375e:	46 88 3c 28                                     	mov    %r15b,(%rax,%r13,1)
  1d3762:	49 ff c5                                        	inc    %r13
  1d3765:	4c 89 6d 10                                     	mov    %r13,0x10(%rbp)
  1d3769:	41 80 ff ff                                     	cmp    $0xff,%r15b
  1d376d:	0f 94 c0                                        	sete   %al
  1d3770:	b1 08                                           	mov    $0x8,%cl
  1d3772:	28 c1                                           	sub    %al,%cl
  1d3774:	88 4c 24 6b                                     	mov    %cl,0x6b(%rsp)
  1d3778:	88 4c 24 6c                                     	mov    %cl,0x6c(%rsp)
  1d377c:	c6 44 24 6a 00                                  	movb   $0x0,0x6a(%rsp)
  1d3781:	48 8b ac 24 e0 00 00 00                         	mov    0xe0(%rsp),%rbp
  1d3789:	48 85 ed                                        	test   %rbp,%rbp
  1d378c:	74 2b                                           	je     1d37b9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa99>
  1d378e:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d3793:	4c 8b 78 10                                     	mov    0x10(%rax),%r15
  1d3797:	4c 2b 7c 24 28                                  	sub    0x28(%rsp),%r15
  1d379c:	4c 8b 6d 10                                     	mov    0x10(%rbp),%r13
  1d37a0:	4c 3b 6d 00                                     	cmp    0x0(%rbp),%r13
  1d37a4:	0f 84 9c 00 00 00                               	je     1d3846 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xb26>
  1d37aa:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1d37ae:	4e 89 3c e8                                     	mov    %r15,(%rax,%r13,8)
  1d37b2:	49 ff c5                                        	inc    %r13
  1d37b5:	4c 89 6d 10                                     	mov    %r13,0x10(%rbp)
  1d37b9:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d37bf:	4c 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%r15
  1d37c7:	0f 83 33 fc ff ff                               	jae    1d3400 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6e0>
  1d37cd:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d37d2:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d37d6:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1d37db:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d37df:	0f 84 04 fc ff ff                               	je     1d33e9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6c9>
  1d37e5:	66 83 fb 09                                     	cmp    $0x9,%bx
  1d37e9:	0f 82 fa fb ff ff                               	jb     1d33e9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6c9>
  1d37ef:	41 0f b7 c6                                     	movzwl %r14w,%eax
  1d37f3:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d37f9:	c1 e8 11                                        	shr    $0x11,%eax
  1d37fc:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d37ff:	44 89 f1                                        	mov    %r14d,%ecx
  1d3802:	29 c1                                           	sub    %eax,%ecx
  1d3804:	66 85 c9                                        	test   %cx,%cx
  1d3807:	0f 84 dc fb ff ff                               	je     1d33e9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6c9>
  1d380d:	66 c7 44 24 6a 00 08                            	movw   $0x800,0x6a(%rsp)
  1d3814:	c6 44 24 6c 08                                  	movb   $0x8,0x6c(%rsp)
  1d3819:	e9 e2 fb ff ff                                  	jmp    1d3400 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6e0>
  1d381e:	48 8b bc 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdi
  1d3826:	4c 89 f0                                        	mov    %r14,%rax
  1d3829:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d3830:	4c 39 ff                                        	cmp    %r15,%rdi
  1d3833:	0f 87 4b 0b 00 00                               	ja     1d4384 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1664>
  1d3839:	4c 01 c7                                        	add    %r8,%rdi
  1d383c:	48 ff c8                                        	dec    %rax
  1d383f:	75 ef                                           	jne    1d3830 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xb10>
  1d3841:	e9 e5 fd ff ff                                  	jmp    1d362b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x90b>
  1d3846:	48 89 ef                                        	mov    %rbp,%rdi
  1d3849:	ff 15 09 1a 0a 00                               	call   *0xa1a09(%rip)        # 275258 <_DYNAMIC+0x6a8>
  1d384f:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3854:	e9 51 ff ff ff                                  	jmp    1d37aa <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa8a>
  1d3859:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
  1d385e:	48 8b 48 40                                     	mov    0x40(%rax),%rcx
  1d3862:	48 89 8c 24 88 00 00 00                         	mov    %rcx,0x88(%rsp)
  1d386a:	4c 8b 70 38                                     	mov    0x38(%rax),%r14
  1d386e:	48 8b 08                                        	mov    (%rax),%rcx
  1d3871:	48 89 8c 24 80 00 00 00                         	mov    %rcx,0x80(%rsp)
  1d3879:	4c 8b 60 08                                     	mov    0x8(%rax),%r12
  1d387d:	48 8b 78 30                                     	mov    0x30(%rax),%rdi
  1d3881:	48 89 7c 24 78                                  	mov    %rdi,0x78(%rsp)
  1d3886:	48 8d 04 7f                                     	lea    (%rdi,%rdi,2),%rax
  1d388a:	48 83 c0 fd                                     	add    $0xfffffffffffffffd,%rax
  1d388e:	48 b9 ab aa aa aa aa aa aa aa                   	movabs $0xaaaaaaaaaaaaaaab,%rcx
  1d3898:	48 89 84 24 b0 00 00 00                         	mov    %rax,0xb0(%rsp)
  1d38a0:	48 f7 e1                                        	mul    %rcx
  1d38a3:	48 8d 04 7f                                     	lea    (%rdi,%rdi,2),%rax
  1d38a7:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
  1d38af:	48 d1 ea                                        	shr    $1,%rdx
  1d38b2:	8d 42 01                                        	lea    0x1(%rdx),%eax
  1d38b5:	83 e0 07                                        	and    $0x7,%eax
  1d38b8:	83 e2 07                                        	and    $0x7,%edx
  1d38bb:	48 89 94 24 a8 00 00 00                         	mov    %rdx,0xa8(%rsp)
  1d38c3:	48 f7 d8                                        	neg    %rax
  1d38c6:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
  1d38ce:	45 31 ff                                        	xor    %r15d,%r15d
  1d38d1:	48 8d 84 24 f8 00 00 00                         	lea    0xf8(%rsp),%rax
  1d38d9:	48 89 84 24 d8 00 00 00                         	mov    %rax,0xd8(%rsp)
  1d38e1:	4c 89 a4 24 90 00 00 00                         	mov    %r12,0x90(%rsp)
  1d38e9:	eb 25                                           	jmp    1d3910 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbf0>
  1d38eb:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d38f4:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d38fc:	0f 1f 40 00                                     	nopl   0x0(%rax)
  1d3900:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d3906:	8b 74 24 74                                     	mov    0x74(%rsp),%esi
  1d390a:	0f 84 6a 09 00 00                               	je     1d427a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x155a>
  1d3910:	41 0f b7 c7                                     	movzwl %r15w,%eax
  1d3914:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d391a:	c1 e8 11                                        	shr    $0x11,%eax
  1d391d:	8d 0c 40                                        	lea    (%rax,%rax,2),%ecx
  1d3920:	31 d2                                           	xor    %edx,%edx
  1d3922:	44 89 fb                                        	mov    %r15d,%ebx
  1d3925:	66 29 cb                                        	sub    %cx,%bx
  1d3928:	0f 95 c2                                        	setne  %dl
  1d392b:	01 c2                                           	add    %eax,%edx
  1d392d:	b0 03                                           	mov    $0x3,%al
  1d392f:	89 f1                                           	mov    %esi,%ecx
  1d3931:	28 d1                                           	sub    %dl,%cl
  1d3933:	0f 82 30 09 00 00                               	jb     1d4269 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1549>
  1d3939:	44 89 fd                                        	mov    %r15d,%ebp
  1d393c:	66 85 db                                        	test   %bx,%bx
  1d393f:	0f 95 c2                                        	setne  %dl
  1d3942:	66 41 83 ff 0a                                  	cmp    $0xa,%r15w
  1d3947:	41 0f 93 c5                                     	setae  %r13b
  1d394b:	45 20 d5                                        	and    %r10b,%r13b
  1d394e:	41 20 d5                                        	and    %dl,%r13b
  1d3951:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  1d3956:	88 4a 48                                        	mov    %cl,0x48(%rdx)
  1d3959:	66 85 db                                        	test   %bx,%bx
  1d395c:	74 32                                           	je     1d3990 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xc70>
  1d395e:	0f b7 c3                                        	movzwl %bx,%eax
  1d3961:	83 f8 01                                        	cmp    $0x1,%eax
  1d3964:	0f 85 86 01 00 00                               	jne    1d3af0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xdd0>
  1d396a:	45 84 ed                                        	test   %r13b,%r13b
  1d396d:	0f 84 93 01 00 00                               	je     1d3b06 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xde6>
  1d3973:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d3978:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d397d:	e8 ae 87 fe ff                                  	call   1bc130 <emuella_j2k_tier1::significance_propagation_pass_encode_raw::<true>>
  1d3982:	e9 9f 01 00 00                                  	jmp    1d3b26 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe06>
  1d3987:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d3990:	45 84 ed                                        	test   %r13b,%r13b
  1d3993:	4c 8b bc 24 88 00 00 00                         	mov    0x88(%rsp),%r15
  1d399b:	0f 85 15 09 00 00                               	jne    1d42b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1596>
  1d39a1:	48 8d bc 24 b8 00 00 00                         	lea    0xb8(%rsp),%rdi
  1d39a9:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1d39ae:	48 8d 54 24 28                                  	lea    0x28(%rsp),%rdx
  1d39b3:	e8 88 ec fd ff                                  	call   1b2640 <emuella_j2k_tier1::cleanup_pass_encode::<true>>
  1d39b8:	0f b6 84 24 b8 00 00 00                         	movzbl 0xb8(%rsp),%eax
  1d39c0:	3c ff                                           	cmp    $0xff,%al
  1d39c2:	0f 85 3b 09 00 00                               	jne    1d4303 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x15e3>
  1d39c8:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d39cd:	41 f6 c2 20                                     	test   $0x20,%r10b
  1d39d1:	74 4f                                           	je     1d3a22 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd02>
  1d39d3:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d39d8:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d39dd:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d39e2:	e8 19 7c 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d39e7:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d39ec:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d39f1:	31 d2                                           	xor    %edx,%edx
  1d39f3:	e8 08 7c 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d39f8:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d39fd:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3a02:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d3a07:	e8 f4 7b 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d3a0c:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d3a11:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3a16:	31 d2                                           	xor    %edx,%edx
  1d3a18:	e8 e3 7b 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d3a1d:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3a22:	4d 85 ff                                        	test   %r15,%r15
  1d3a25:	44 0f b6 44 24 20                               	movzbl 0x20(%rsp),%r8d
  1d3a2b:	4c 8b 8c 24 80 00 00 00                         	mov    0x80(%rsp),%r9
  1d3a33:	4c 8b 5c 24 78                                  	mov    0x78(%rsp),%r11
  1d3a38:	0f 84 f3 00 00 00                               	je     1d3b31 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe11>
  1d3a3e:	4d 85 db                                        	test   %r11,%r11
  1d3a41:	0f 84 c6 02 00 00                               	je     1d3d0d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xfed>
  1d3a47:	31 c0                                           	xor    %eax,%eax
  1d3a49:	eb 0e                                           	jmp    1d3a59 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd39>
  1d3a4b:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
  1d3a50:	4c 39 f8                                        	cmp    %r15,%rax
  1d3a53:	0f 84 d8 00 00 00                               	je     1d3b31 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe11>
  1d3a59:	48 ff c0                                        	inc    %rax
  1d3a5c:	48 89 c7                                        	mov    %rax,%rdi
  1d3a5f:	49 0f af fe                                     	imul   %r14,%rdi
  1d3a63:	48 ff c7                                        	inc    %rdi
  1d3a66:	48 89 fe                                        	mov    %rdi,%rsi
  1d3a69:	4c 01 de                                        	add    %r11,%rsi
  1d3a6c:	0f 82 3b 09 00 00                               	jb     1d43ad <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x168d>
  1d3a72:	4c 39 e6                                        	cmp    %r12,%rsi
  1d3a75:	0f 87 32 09 00 00                               	ja     1d43ad <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x168d>
  1d3a7b:	48 8d 0c 7f                                     	lea    (%rdi,%rdi,2),%rcx
  1d3a7f:	4c 01 c9                                        	add    %r9,%rcx
  1d3a82:	48 89 ca                                        	mov    %rcx,%rdx
  1d3a85:	83 bc 24 a8 00 00 00 07                         	cmpl   $0x7,0xa8(%rsp)
  1d3a8d:	74 1e                                           	je     1d3aad <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd8d>
  1d3a8f:	48 8b b4 24 98 00 00 00                         	mov    0x98(%rsp),%rsi
  1d3a97:	48 89 ca                                        	mov    %rcx,%rdx
  1d3a9a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d3aa0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d3aa4:	48 83 c2 03                                     	add    $0x3,%rdx
  1d3aa8:	48 ff c6                                        	inc    %rsi
  1d3aab:	75 f3                                           	jne    1d3aa0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd80>
  1d3aad:	48 83 bc 24 b0 00 00 00 15                      	cmpq   $0x15,0xb0(%rsp)
  1d3ab6:	72 98                                           	jb     1d3a50 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd30>
  1d3ab8:	48 03 8c 24 a0 00 00 00                         	add    0xa0(%rsp),%rcx
  1d3ac0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d3ac4:	c6 42 04 00                                     	movb   $0x0,0x4(%rdx)
  1d3ac8:	c6 42 07 00                                     	movb   $0x0,0x7(%rdx)
  1d3acc:	c6 42 0a 00                                     	movb   $0x0,0xa(%rdx)
  1d3ad0:	c6 42 0d 00                                     	movb   $0x0,0xd(%rdx)
  1d3ad4:	c6 42 10 00                                     	movb   $0x0,0x10(%rdx)
  1d3ad8:	c6 42 13 00                                     	movb   $0x0,0x13(%rdx)
  1d3adc:	c6 42 16 00                                     	movb   $0x0,0x16(%rdx)
  1d3ae0:	48 83 c2 18                                     	add    $0x18,%rdx
  1d3ae4:	48 39 ca                                        	cmp    %rcx,%rdx
  1d3ae7:	75 d7                                           	jne    1d3ac0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xda0>
  1d3ae9:	e9 62 ff ff ff                                  	jmp    1d3a50 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd30>
  1d3aee:	66 90                                           	xchg   %ax,%ax
  1d3af0:	45 84 ed                                        	test   %r13b,%r13b
  1d3af3:	74 22                                           	je     1d3b17 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xdf7>
  1d3af5:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d3afa:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d3aff:	e8 bc 08 00 00                                  	call   1d43c0 <emuella_j2k_tier1::magnitude_refinement_pass_encode_raw>
  1d3b04:	eb 20                                           	jmp    1d3b26 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe06>
  1d3b06:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d3b0b:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d3b10:	e8 fb 6f fe ff                                  	call   1bab10 <emuella_j2k_tier1::significance_propagation_pass_encode::<true>>
  1d3b15:	eb 0f                                           	jmp    1d3b26 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe06>
  1d3b17:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d3b1c:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d3b21:	e8 da 4e fe ff                                  	call   1b8a00 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<true>>
  1d3b26:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3b2b:	44 0f b6 44 24 20                               	movzbl 0x20(%rsp),%r8d
  1d3b31:	44 8d 7d 01                                     	lea    0x1(%rbp),%r15d
  1d3b35:	41 f6 c2 04                                     	test   $0x4,%r10b
  1d3b39:	75 35                                           	jne    1d3b70 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe50>
  1d3b3b:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d3b41:	74 2d                                           	je     1d3b70 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe50>
  1d3b43:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d3b47:	0f 84 b3 fd ff ff                               	je     1d3900 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbe0>
  1d3b4d:	66 83 fd 09                                     	cmp    $0x9,%bp
  1d3b51:	74 1d                                           	je     1d3b70 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe50>
  1d3b53:	0f 96 c0                                        	setbe  %al
  1d3b56:	66 83 fb 01                                     	cmp    $0x1,%bx
  1d3b5a:	0f 94 c1                                        	sete   %cl
  1d3b5d:	08 c1                                           	or     %al,%cl
  1d3b5f:	0f 85 9b fd ff ff                               	jne    1d3900 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbe0>
  1d3b65:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  1d3b70:	45 84 ed                                        	test   %r13b,%r13b
  1d3b73:	74 3b                                           	je     1d3bb0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe90>
  1d3b75:	0f b6 44 24 6c                                  	movzbl 0x6c(%rsp),%eax
  1d3b7a:	0f b6 54 24 6b                                  	movzbl 0x6b(%rsp),%edx
  1d3b7f:	48 8b 4c 24 28                                  	mov    0x28(%rsp),%rcx
  1d3b84:	48 8b 5c 24 30                                  	mov    0x30(%rsp),%rbx
  1d3b89:	4c 8b 6b 10                                     	mov    0x10(%rbx),%r13
  1d3b8d:	49 39 cd                                        	cmp    %rcx,%r13
  1d3b90:	76 36                                           	jbe    1d3bc8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xea8>
  1d3b92:	38 d0                                           	cmp    %dl,%al
  1d3b94:	72 3a                                           	jb     1d3bd0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xeb0>
  1d3b96:	48 8b 53 08                                     	mov    0x8(%rbx),%rdx
  1d3b9a:	48 01 ca                                        	add    %rcx,%rdx
  1d3b9d:	4c 01 ea                                        	add    %r13,%rdx
  1d3ba0:	48 f7 d1                                        	not    %rcx
  1d3ba3:	80 3c 11 ff                                     	cmpb   $0xff,(%rcx,%rdx,1)
  1d3ba7:	0f 85 c3 00 00 00                               	jne    1d3c70 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf50>
  1d3bad:	eb 21                                           	jmp    1d3bd0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xeb0>
  1d3baf:	90                                              	nop
  1d3bb0:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d3bb5:	8b 74 24 70                                     	mov    0x70(%rsp),%esi
  1d3bb9:	e8 82 e6 ff ff                                  	call   1d2240 <emuella_j2k_tier1::terminate_arithmetic_segment>
  1d3bbe:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3bc3:	e9 a8 00 00 00                                  	jmp    1d3c70 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf50>
  1d3bc8:	38 d0                                           	cmp    %dl,%al
  1d3bca:	0f 83 a0 00 00 00                               	jae    1d3c70 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf50>
  1d3bd0:	44 0f b6 64 24 6a                               	movzbl 0x6a(%rsp),%r12d
  1d3bd6:	84 c0                                           	test   %al,%al
  1d3bd8:	74 5c                                           	je     1d3c36 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf16>
  1d3bda:	89 c2                                           	mov    %eax,%edx
  1d3bdc:	80 e2 03                                        	and    $0x3,%dl
  1d3bdf:	3c 04                                           	cmp    $0x4,%al
  1d3be1:	72 36                                           	jb     1d3c19 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xef9>
  1d3be3:	89 c6                                           	mov    %eax,%esi
  1d3be5:	40 80 e6 fc                                     	and    $0xfc,%sil
  1d3be9:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d3bf0:	8d 48 06                                        	lea    0x6(%rax),%ecx
  1d3bf3:	80 e1 07                                        	and    $0x7,%cl
  1d3bf6:	44 89 c7                                        	mov    %r8d,%edi
  1d3bf9:	40 d2 e7                                        	shl    %cl,%dil
  1d3bfc:	44 08 e7                                        	or     %r12b,%dil
  1d3bff:	04 fc                                           	add    $0xfc,%al
  1d3c01:	89 c1                                           	mov    %eax,%ecx
  1d3c03:	80 e1 07                                        	and    $0x7,%cl
  1d3c06:	45 89 c4                                        	mov    %r8d,%r12d
  1d3c09:	41 d2 e4                                        	shl    %cl,%r12b
  1d3c0c:	41 08 fc                                        	or     %dil,%r12b
  1d3c0f:	40 80 c6 fc                                     	add    $0xfc,%sil
  1d3c13:	75 db                                           	jne    1d3bf0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xed0>
  1d3c15:	84 d2                                           	test   %dl,%dl
  1d3c17:	74 1d                                           	je     1d3c36 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf16>
  1d3c19:	fe c8                                           	dec    %al
  1d3c1b:	31 f6                                           	xor    %esi,%esi
  1d3c1d:	0f 1f 00                                        	nopl   (%rax)
  1d3c20:	89 c1                                           	mov    %eax,%ecx
  1d3c22:	80 e1 07                                        	and    $0x7,%cl
  1d3c25:	89 f7                                           	mov    %esi,%edi
  1d3c27:	40 d2 e7                                        	shl    %cl,%dil
  1d3c2a:	41 08 fc                                        	or     %dil,%r12b
  1d3c2d:	44 30 c6                                        	xor    %r8b,%sil
  1d3c30:	fe c8                                           	dec    %al
  1d3c32:	fe ca                                           	dec    %dl
  1d3c34:	75 ea                                           	jne    1d3c20 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf00>
  1d3c36:	4c 3b 2b                                        	cmp    (%rbx),%r13
  1d3c39:	75 0e                                           	jne    1d3c49 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf29>
  1d3c3b:	48 89 df                                        	mov    %rbx,%rdi
  1d3c3e:	ff 15 fc 11 0a 00                               	call   *0xa11fc(%rip)        # 274e40 <_DYNAMIC+0x290>
  1d3c44:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3c49:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1d3c4d:	46 88 24 28                                     	mov    %r12b,(%rax,%r13,1)
  1d3c51:	49 ff c5                                        	inc    %r13
  1d3c54:	4c 89 6b 10                                     	mov    %r13,0x10(%rbx)
  1d3c58:	41 80 fc ff                                     	cmp    $0xff,%r12b
  1d3c5c:	0f 94 c0                                        	sete   %al
  1d3c5f:	b1 08                                           	mov    $0x8,%cl
  1d3c61:	28 c1                                           	sub    %al,%cl
  1d3c63:	88 4c 24 6b                                     	mov    %cl,0x6b(%rsp)
  1d3c67:	88 4c 24 6c                                     	mov    %cl,0x6c(%rsp)
  1d3c6b:	c6 44 24 6a 00                                  	movb   $0x0,0x6a(%rsp)
  1d3c70:	4c 8b ac 24 e0 00 00 00                         	mov    0xe0(%rsp),%r13
  1d3c78:	4d 85 ed                                        	test   %r13,%r13
  1d3c7b:	74 2b                                           	je     1d3ca8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf88>
  1d3c7d:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d3c82:	48 8b 58 10                                     	mov    0x10(%rax),%rbx
  1d3c86:	48 2b 5c 24 28                                  	sub    0x28(%rsp),%rbx
  1d3c8b:	4d 8b 65 10                                     	mov    0x10(%r13),%r12
  1d3c8f:	4d 3b 65 00                                     	cmp    0x0(%r13),%r12
  1d3c93:	0f 84 9d 00 00 00                               	je     1d3d36 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1016>
  1d3c99:	49 8b 45 08                                     	mov    0x8(%r13),%rax
  1d3c9d:	4a 89 1c e0                                     	mov    %rbx,(%rax,%r12,8)
  1d3ca1:	49 ff c4                                        	inc    %r12
  1d3ca4:	4d 89 65 10                                     	mov    %r12,0x10(%r13)
  1d3ca8:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d3cae:	4c 8b a4 24 90 00 00 00                         	mov    0x90(%rsp),%r12
  1d3cb6:	0f 83 44 fc ff ff                               	jae    1d3900 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbe0>
  1d3cbc:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d3cc1:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d3cc5:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1d3cca:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d3cce:	0f 84 17 fc ff ff                               	je     1d38eb <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbcb>
  1d3cd4:	66 83 fd 09                                     	cmp    $0x9,%bp
  1d3cd8:	0f 82 0d fc ff ff                               	jb     1d38eb <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbcb>
  1d3cde:	41 0f b7 c7                                     	movzwl %r15w,%eax
  1d3ce2:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d3ce8:	c1 e8 11                                        	shr    $0x11,%eax
  1d3ceb:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d3cee:	44 89 f9                                        	mov    %r15d,%ecx
  1d3cf1:	29 c1                                           	sub    %eax,%ecx
  1d3cf3:	66 85 c9                                        	test   %cx,%cx
  1d3cf6:	0f 84 ef fb ff ff                               	je     1d38eb <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbcb>
  1d3cfc:	66 c7 44 24 6a 00 08                            	movw   $0x800,0x6a(%rsp)
  1d3d03:	c6 44 24 6c 08                                  	movb   $0x8,0x6c(%rsp)
  1d3d08:	e9 f3 fb ff ff                                  	jmp    1d3900 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbe0>
  1d3d0d:	49 8d 7e 01                                     	lea    0x1(%r14),%rdi
  1d3d11:	4c 89 f8                                        	mov    %r15,%rax
  1d3d14:	66 66 66 2e 0f 1f 84 00 00 00 00 00             	data16 data16 cs nopw 0x0(%rax,%rax,1)
  1d3d20:	4c 39 e7                                        	cmp    %r12,%rdi
  1d3d23:	0f 87 81 06 00 00                               	ja     1d43aa <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x168a>
  1d3d29:	4c 01 f7                                        	add    %r14,%rdi
  1d3d2c:	48 ff c8                                        	dec    %rax
  1d3d2f:	75 ef                                           	jne    1d3d20 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1000>
  1d3d31:	e9 fb fd ff ff                                  	jmp    1d3b31 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe11>
  1d3d36:	4c 89 ef                                        	mov    %r13,%rdi
  1d3d39:	ff 15 19 15 0a 00                               	call   *0xa1519(%rip)        # 275258 <_DYNAMIC+0x6a8>
  1d3d3f:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3d44:	e9 50 ff ff ff                                  	jmp    1d3c99 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf79>
  1d3d49:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
  1d3d4e:	48 8b 48 40                                     	mov    0x40(%rax),%rcx
  1d3d52:	48 89 8c 24 90 00 00 00                         	mov    %rcx,0x90(%rsp)
  1d3d5a:	48 8b 78 38                                     	mov    0x38(%rax),%rdi
  1d3d5e:	48 89 bc 24 88 00 00 00                         	mov    %rdi,0x88(%rsp)
  1d3d66:	48 8b 08                                        	mov    (%rax),%rcx
  1d3d69:	48 89 8c 24 80 00 00 00                         	mov    %rcx,0x80(%rsp)
  1d3d71:	4c 8b 78 08                                     	mov    0x8(%rax),%r15
  1d3d75:	48 8b 40 30                                     	mov    0x30(%rax),%rax
  1d3d79:	48 89 44 24 78                                  	mov    %rax,0x78(%rsp)
  1d3d7e:	48 8d 0c 40                                     	lea    (%rax,%rax,2),%rcx
  1d3d82:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  1d3d87:	45 89 d4                                        	mov    %r10d,%r12d
  1d3d8a:	41 c0 ec 04                                     	shr    $0x4,%r12b
  1d3d8e:	41 80 e4 01                                     	and    $0x1,%r12b
  1d3d92:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
  1d3d96:	48 83 c0 fd                                     	add    $0xfffffffffffffffd,%rax
  1d3d9a:	48 b9 ab aa aa aa aa aa aa aa                   	movabs $0xaaaaaaaaaaaaaaab,%rcx
  1d3da4:	48 89 84 24 b0 00 00 00                         	mov    %rax,0xb0(%rsp)
  1d3dac:	48 f7 e1                                        	mul    %rcx
  1d3daf:	48 d1 ea                                        	shr    $1,%rdx
  1d3db2:	8d 42 01                                        	lea    0x1(%rdx),%eax
  1d3db5:	83 e0 07                                        	and    $0x7,%eax
  1d3db8:	83 e2 07                                        	and    $0x7,%edx
  1d3dbb:	48 89 94 24 a8 00 00 00                         	mov    %rdx,0xa8(%rsp)
  1d3dc3:	48 f7 d8                                        	neg    %rax
  1d3dc6:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
  1d3dce:	48 8d 47 01                                     	lea    0x1(%rdi),%rax
  1d3dd2:	48 89 84 24 d0 00 00 00                         	mov    %rax,0xd0(%rsp)
  1d3dda:	45 31 f6                                        	xor    %r14d,%r14d
  1d3ddd:	48 8d 84 24 00 01 00 00                         	lea    0x100(%rsp),%rax
  1d3de5:	48 89 84 24 d8 00 00 00                         	mov    %rax,0xd8(%rsp)
  1d3ded:	4c 89 bc 24 98 00 00 00                         	mov    %r15,0x98(%rsp)
  1d3df5:	eb 29                                           	jmp    1d3e20 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1100>
  1d3df7:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d3e00:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d3e08:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
  1d3e10:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d3e16:	8b 74 24 74                                     	mov    0x74(%rsp),%esi
  1d3e1a:	0f 84 5a 04 00 00                               	je     1d427a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x155a>
  1d3e20:	41 0f b7 c6                                     	movzwl %r14w,%eax
  1d3e24:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d3e2a:	c1 e8 11                                        	shr    $0x11,%eax
  1d3e2d:	8d 0c 40                                        	lea    (%rax,%rax,2),%ecx
  1d3e30:	31 d2                                           	xor    %edx,%edx
  1d3e32:	44 89 f5                                        	mov    %r14d,%ebp
  1d3e35:	66 29 cd                                        	sub    %cx,%bp
  1d3e38:	0f 95 c2                                        	setne  %dl
  1d3e3b:	01 c2                                           	add    %eax,%edx
  1d3e3d:	b0 03                                           	mov    $0x3,%al
  1d3e3f:	89 f1                                           	mov    %esi,%ecx
  1d3e41:	28 d1                                           	sub    %dl,%cl
  1d3e43:	0f 82 20 04 00 00                               	jb     1d4269 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1549>
  1d3e49:	44 89 f3                                        	mov    %r14d,%ebx
  1d3e4c:	66 85 ed                                        	test   %bp,%bp
  1d3e4f:	0f 95 c2                                        	setne  %dl
  1d3e52:	66 41 83 fe 0a                                  	cmp    $0xa,%r14w
  1d3e57:	41 0f 93 c5                                     	setae  %r13b
  1d3e5b:	45 20 d5                                        	and    %r10b,%r13b
  1d3e5e:	41 20 d5                                        	and    %dl,%r13b
  1d3e61:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  1d3e66:	88 4a 48                                        	mov    %cl,0x48(%rdx)
  1d3e69:	66 85 ed                                        	test   %bp,%bp
  1d3e6c:	74 32                                           	je     1d3ea0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1180>
  1d3e6e:	0f b7 c5                                        	movzwl %bp,%eax
  1d3e71:	83 f8 01                                        	cmp    $0x1,%eax
  1d3e74:	0f 85 86 01 00 00                               	jne    1d4000 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x12e0>
  1d3e7a:	45 84 ed                                        	test   %r13b,%r13b
  1d3e7d:	0f 84 93 01 00 00                               	je     1d4016 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x12f6>
  1d3e83:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d3e88:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d3e8d:	e8 9e 82 fe ff                                  	call   1bc130 <emuella_j2k_tier1::significance_propagation_pass_encode_raw::<true>>
  1d3e92:	e9 9f 01 00 00                                  	jmp    1d4036 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1316>
  1d3e97:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d3ea0:	45 84 ed                                        	test   %r13b,%r13b
  1d3ea3:	4c 8b b4 24 90 00 00 00                         	mov    0x90(%rsp),%r14
  1d3eab:	0f 85 05 04 00 00                               	jne    1d42b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1596>
  1d3eb1:	48 8d bc 24 b8 00 00 00                         	lea    0xb8(%rsp),%rdi
  1d3eb9:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1d3ebe:	48 8d 54 24 28                                  	lea    0x28(%rsp),%rdx
  1d3ec3:	e8 78 e7 fd ff                                  	call   1b2640 <emuella_j2k_tier1::cleanup_pass_encode::<true>>
  1d3ec8:	0f b6 84 24 b8 00 00 00                         	movzbl 0xb8(%rsp),%eax
  1d3ed0:	3c ff                                           	cmp    $0xff,%al
  1d3ed2:	0f 85 49 04 00 00                               	jne    1d4321 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1601>
  1d3ed8:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3edd:	41 f6 c2 20                                     	test   $0x20,%r10b
  1d3ee1:	74 4f                                           	je     1d3f32 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1212>
  1d3ee3:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d3ee8:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3eed:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d3ef2:	e8 09 77 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d3ef7:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d3efc:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3f01:	31 d2                                           	xor    %edx,%edx
  1d3f03:	e8 f8 76 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d3f08:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d3f0d:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3f12:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d3f17:	e8 e4 76 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d3f1c:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d3f21:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d3f26:	31 d2                                           	xor    %edx,%edx
  1d3f28:	e8 d3 76 00 00                                  	call   1db600 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d3f2d:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d3f32:	4d 85 f6                                        	test   %r14,%r14
  1d3f35:	4c 8b 84 24 88 00 00 00                         	mov    0x88(%rsp),%r8
  1d3f3d:	4c 8b 8c 24 80 00 00 00                         	mov    0x80(%rsp),%r9
  1d3f45:	4c 8b 5c 24 78                                  	mov    0x78(%rsp),%r11
  1d3f4a:	0f 84 eb 00 00 00                               	je     1d403b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x131b>
  1d3f50:	4d 85 db                                        	test   %r11,%r11
  1d3f53:	0f 84 d5 02 00 00                               	je     1d422e <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x150e>
  1d3f59:	31 c0                                           	xor    %eax,%eax
  1d3f5b:	eb 0c                                           	jmp    1d3f69 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1249>
  1d3f5d:	0f 1f 00                                        	nopl   (%rax)
  1d3f60:	4c 39 f0                                        	cmp    %r14,%rax
  1d3f63:	0f 84 d2 00 00 00                               	je     1d403b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x131b>
  1d3f69:	48 ff c0                                        	inc    %rax
  1d3f6c:	48 89 c7                                        	mov    %rax,%rdi
  1d3f6f:	49 0f af f8                                     	imul   %r8,%rdi
  1d3f73:	48 ff c7                                        	inc    %rdi
  1d3f76:	48 89 fe                                        	mov    %rdi,%rsi
  1d3f79:	4c 01 de                                        	add    %r11,%rsi
  1d3f7c:	0f 82 05 04 00 00                               	jb     1d4387 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1667>
  1d3f82:	4c 39 fe                                        	cmp    %r15,%rsi
  1d3f85:	0f 87 fc 03 00 00                               	ja     1d4387 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1667>
  1d3f8b:	48 8d 0c 7f                                     	lea    (%rdi,%rdi,2),%rcx
  1d3f8f:	4c 01 c9                                        	add    %r9,%rcx
  1d3f92:	48 89 ca                                        	mov    %rcx,%rdx
  1d3f95:	83 bc 24 a8 00 00 00 07                         	cmpl   $0x7,0xa8(%rsp)
  1d3f9d:	74 1e                                           	je     1d3fbd <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x129d>
  1d3f9f:	48 8b b4 24 a0 00 00 00                         	mov    0xa0(%rsp),%rsi
  1d3fa7:	48 89 ca                                        	mov    %rcx,%rdx
  1d3faa:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d3fb0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d3fb4:	48 83 c2 03                                     	add    $0x3,%rdx
  1d3fb8:	48 ff c6                                        	inc    %rsi
  1d3fbb:	75 f3                                           	jne    1d3fb0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1290>
  1d3fbd:	48 83 bc 24 b0 00 00 00 15                      	cmpq   $0x15,0xb0(%rsp)
  1d3fc6:	72 98                                           	jb     1d3f60 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1240>
  1d3fc8:	48 03 4c 24 20                                  	add    0x20(%rsp),%rcx
  1d3fcd:	0f 1f 00                                        	nopl   (%rax)
  1d3fd0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d3fd4:	c6 42 04 00                                     	movb   $0x0,0x4(%rdx)
  1d3fd8:	c6 42 07 00                                     	movb   $0x0,0x7(%rdx)
  1d3fdc:	c6 42 0a 00                                     	movb   $0x0,0xa(%rdx)
  1d3fe0:	c6 42 0d 00                                     	movb   $0x0,0xd(%rdx)
  1d3fe4:	c6 42 10 00                                     	movb   $0x0,0x10(%rdx)
  1d3fe8:	c6 42 13 00                                     	movb   $0x0,0x13(%rdx)
  1d3fec:	c6 42 16 00                                     	movb   $0x0,0x16(%rdx)
  1d3ff0:	48 83 c2 18                                     	add    $0x18,%rdx
  1d3ff4:	48 39 ca                                        	cmp    %rcx,%rdx
  1d3ff7:	75 d7                                           	jne    1d3fd0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x12b0>
  1d3ff9:	e9 62 ff ff ff                                  	jmp    1d3f60 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1240>
  1d3ffe:	66 90                                           	xchg   %ax,%ax
  1d4000:	45 84 ed                                        	test   %r13b,%r13b
  1d4003:	74 22                                           	je     1d4027 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1307>
  1d4005:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d400a:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d400f:	e8 ac 03 00 00                                  	call   1d43c0 <emuella_j2k_tier1::magnitude_refinement_pass_encode_raw>
  1d4014:	eb 20                                           	jmp    1d4036 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1316>
  1d4016:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d401b:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d4020:	e8 eb 6a fe ff                                  	call   1bab10 <emuella_j2k_tier1::significance_propagation_pass_encode::<true>>
  1d4025:	eb 0f                                           	jmp    1d4036 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1316>
  1d4027:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d402c:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d4031:	e8 ca 49 fe ff                                  	call   1b8a00 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<true>>
  1d4036:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d403b:	44 8d 73 01                                     	lea    0x1(%rbx),%r14d
  1d403f:	c6 44 24 44 04                                  	movb   $0x4,0x44(%rsp)
  1d4044:	48 8d 44 24 45                                  	lea    0x45(%rsp),%rax
  1d4049:	0f 57 c0                                        	xorps  %xmm0,%xmm0
  1d404c:	0f 11 40 10                                     	movups %xmm0,0x10(%rax)
  1d4050:	0f 11 00                                        	movups %xmm0,(%rax)
  1d4053:	c6 40 20 00                                     	movb   $0x0,0x20(%rax)
  1d4057:	c7 44 24 66 03 00 2e 00                         	movl   $0x2e0003,0x66(%rsp)
  1d405f:	41 f6 c2 04                                     	test   $0x4,%r10b
  1d4063:	75 2b                                           	jne    1d4090 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1370>
  1d4065:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d406b:	74 23                                           	je     1d4090 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1370>
  1d406d:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d4071:	0f 84 99 fd ff ff                               	je     1d3e10 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10f0>
  1d4077:	66 83 fb 09                                     	cmp    $0x9,%bx
  1d407b:	74 13                                           	je     1d4090 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1370>
  1d407d:	0f 96 c0                                        	setbe  %al
  1d4080:	66 83 fd 01                                     	cmp    $0x1,%bp
  1d4084:	0f 94 c1                                        	sete   %cl
  1d4087:	08 c1                                           	or     %al,%cl
  1d4089:	0f 85 81 fd ff ff                               	jne    1d3e10 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10f0>
  1d408f:	90                                              	nop
  1d4090:	45 84 ed                                        	test   %r13b,%r13b
  1d4093:	74 3b                                           	je     1d40d0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13b0>
  1d4095:	0f b6 44 24 6c                                  	movzbl 0x6c(%rsp),%eax
  1d409a:	0f b6 54 24 6b                                  	movzbl 0x6b(%rsp),%edx
  1d409f:	48 8b 4c 24 28                                  	mov    0x28(%rsp),%rcx
  1d40a4:	48 8b 6c 24 30                                  	mov    0x30(%rsp),%rbp
  1d40a9:	4c 8b 6d 10                                     	mov    0x10(%rbp),%r13
  1d40ad:	49 39 cd                                        	cmp    %rcx,%r13
  1d40b0:	76 36                                           	jbe    1d40e8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13c8>
  1d40b2:	38 d0                                           	cmp    %dl,%al
  1d40b4:	72 3a                                           	jb     1d40f0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13d0>
  1d40b6:	48 8b 55 08                                     	mov    0x8(%rbp),%rdx
  1d40ba:	48 01 ca                                        	add    %rcx,%rdx
  1d40bd:	4c 01 ea                                        	add    %r13,%rdx
  1d40c0:	48 f7 d1                                        	not    %rcx
  1d40c3:	80 3c 11 ff                                     	cmpb   $0xff,(%rcx,%rdx,1)
  1d40c7:	0f 85 c4 00 00 00                               	jne    1d4191 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1471>
  1d40cd:	eb 21                                           	jmp    1d40f0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13d0>
  1d40cf:	90                                              	nop
  1d40d0:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d40d5:	8b 74 24 70                                     	mov    0x70(%rsp),%esi
  1d40d9:	e8 62 e1 ff ff                                  	call   1d2240 <emuella_j2k_tier1::terminate_arithmetic_segment>
  1d40de:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d40e3:	e9 a9 00 00 00                                  	jmp    1d4191 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1471>
  1d40e8:	38 d0                                           	cmp    %dl,%al
  1d40ea:	0f 83 a1 00 00 00                               	jae    1d4191 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1471>
  1d40f0:	44 0f b6 7c 24 6a                               	movzbl 0x6a(%rsp),%r15d
  1d40f6:	84 c0                                           	test   %al,%al
  1d40f8:	74 5c                                           	je     1d4156 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1436>
  1d40fa:	89 c2                                           	mov    %eax,%edx
  1d40fc:	80 e2 03                                        	and    $0x3,%dl
  1d40ff:	3c 04                                           	cmp    $0x4,%al
  1d4101:	72 36                                           	jb     1d4139 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1419>
  1d4103:	89 c6                                           	mov    %eax,%esi
  1d4105:	40 80 e6 fc                                     	and    $0xfc,%sil
  1d4109:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d4110:	8d 48 06                                        	lea    0x6(%rax),%ecx
  1d4113:	80 e1 07                                        	and    $0x7,%cl
  1d4116:	44 89 e7                                        	mov    %r12d,%edi
  1d4119:	40 d2 e7                                        	shl    %cl,%dil
  1d411c:	44 08 ff                                        	or     %r15b,%dil
  1d411f:	04 fc                                           	add    $0xfc,%al
  1d4121:	89 c1                                           	mov    %eax,%ecx
  1d4123:	80 e1 07                                        	and    $0x7,%cl
  1d4126:	45 89 e7                                        	mov    %r12d,%r15d
  1d4129:	41 d2 e7                                        	shl    %cl,%r15b
  1d412c:	41 08 ff                                        	or     %dil,%r15b
  1d412f:	40 80 c6 fc                                     	add    $0xfc,%sil
  1d4133:	75 db                                           	jne    1d4110 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13f0>
  1d4135:	84 d2                                           	test   %dl,%dl
  1d4137:	74 1d                                           	je     1d4156 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1436>
  1d4139:	fe c8                                           	dec    %al
  1d413b:	31 f6                                           	xor    %esi,%esi
  1d413d:	0f 1f 00                                        	nopl   (%rax)
  1d4140:	89 c1                                           	mov    %eax,%ecx
  1d4142:	80 e1 07                                        	and    $0x7,%cl
  1d4145:	89 f7                                           	mov    %esi,%edi
  1d4147:	40 d2 e7                                        	shl    %cl,%dil
  1d414a:	41 08 ff                                        	or     %dil,%r15b
  1d414d:	44 30 e6                                        	xor    %r12b,%sil
  1d4150:	fe c8                                           	dec    %al
  1d4152:	fe ca                                           	dec    %dl
  1d4154:	75 ea                                           	jne    1d4140 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1420>
  1d4156:	4c 3b 6d 00                                     	cmp    0x0(%rbp),%r13
  1d415a:	75 0e                                           	jne    1d416a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x144a>
  1d415c:	48 89 ef                                        	mov    %rbp,%rdi
  1d415f:	ff 15 db 0c 0a 00                               	call   *0xa0cdb(%rip)        # 274e40 <_DYNAMIC+0x290>
  1d4165:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d416a:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1d416e:	46 88 3c 28                                     	mov    %r15b,(%rax,%r13,1)
  1d4172:	49 ff c5                                        	inc    %r13
  1d4175:	4c 89 6d 10                                     	mov    %r13,0x10(%rbp)
  1d4179:	41 80 ff ff                                     	cmp    $0xff,%r15b
  1d417d:	0f 94 c0                                        	sete   %al
  1d4180:	b1 08                                           	mov    $0x8,%cl
  1d4182:	28 c1                                           	sub    %al,%cl
  1d4184:	88 4c 24 6b                                     	mov    %cl,0x6b(%rsp)
  1d4188:	88 4c 24 6c                                     	mov    %cl,0x6c(%rsp)
  1d418c:	c6 44 24 6a 00                                  	movb   $0x0,0x6a(%rsp)
  1d4191:	48 8b ac 24 e0 00 00 00                         	mov    0xe0(%rsp),%rbp
  1d4199:	48 85 ed                                        	test   %rbp,%rbp
  1d419c:	74 2b                                           	je     1d41c9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x14a9>
  1d419e:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d41a3:	4c 8b 78 10                                     	mov    0x10(%rax),%r15
  1d41a7:	4c 2b 7c 24 28                                  	sub    0x28(%rsp),%r15
  1d41ac:	4c 8b 6d 10                                     	mov    0x10(%rbp),%r13
  1d41b0:	4c 3b 6d 00                                     	cmp    0x0(%rbp),%r13
  1d41b4:	0f 84 9c 00 00 00                               	je     1d4256 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1536>
  1d41ba:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1d41be:	4e 89 3c e8                                     	mov    %r15,(%rax,%r13,8)
  1d41c2:	49 ff c5                                        	inc    %r13
  1d41c5:	4c 89 6d 10                                     	mov    %r13,0x10(%rbp)
  1d41c9:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d41cf:	4c 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%r15
  1d41d7:	0f 83 33 fc ff ff                               	jae    1d3e10 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10f0>
  1d41dd:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d41e2:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d41e6:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1d41eb:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d41ef:	0f 84 02 fc ff ff                               	je     1d3df7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10d7>
  1d41f5:	66 83 fb 09                                     	cmp    $0x9,%bx
  1d41f9:	0f 82 f8 fb ff ff                               	jb     1d3df7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10d7>
  1d41ff:	41 0f b7 c6                                     	movzwl %r14w,%eax
  1d4203:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d4209:	c1 e8 11                                        	shr    $0x11,%eax
  1d420c:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d420f:	44 89 f1                                        	mov    %r14d,%ecx
  1d4212:	29 c1                                           	sub    %eax,%ecx
  1d4214:	66 85 c9                                        	test   %cx,%cx
  1d4217:	0f 84 da fb ff ff                               	je     1d3df7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10d7>
  1d421d:	66 c7 44 24 6a 00 08                            	movw   $0x800,0x6a(%rsp)
  1d4224:	c6 44 24 6c 08                                  	movb   $0x8,0x6c(%rsp)
  1d4229:	e9 e2 fb ff ff                                  	jmp    1d3e10 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10f0>
  1d422e:	48 8b bc 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdi
  1d4236:	4c 89 f0                                        	mov    %r14,%rax
  1d4239:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d4240:	4c 39 ff                                        	cmp    %r15,%rdi
  1d4243:	0f 87 3b 01 00 00                               	ja     1d4384 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1664>
  1d4249:	4c 01 c7                                        	add    %r8,%rdi
  1d424c:	48 ff c8                                        	dec    %rax
  1d424f:	75 ef                                           	jne    1d4240 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1520>
  1d4251:	e9 e5 fd ff ff                                  	jmp    1d403b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x131b>
  1d4256:	48 89 ef                                        	mov    %rbp,%rdi
  1d4259:	ff 15 f9 0f 0a 00                               	call   *0xa0ff9(%rip)        # 275258 <_DYNAMIC+0x6a8>
  1d425f:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d4264:	e9 51 ff ff ff                                  	jmp    1d41ba <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x149a>
  1d4269:	b9 28 00 00 00                                  	mov    $0x28,%ecx
  1d426e:	48 8d 3d bd e0 e4 ff                            	lea    -0x1b1f43(%rip),%rdi        # 22332 <anon.38d8fac72642096468ff551e1950d360.1255.llvm.10326835611459877753+0x4672>
  1d4275:	e9 d3 00 00 00                                  	jmp    1d434d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x162d>
  1d427a:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d427f:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d4283:	48 2b 84 24 18 01 00 00                         	sub    0x118(%rsp),%rax
  1d428b:	48 8b 8c 24 08 01 00 00                         	mov    0x108(%rsp),%rcx
  1d4293:	48 89 41 08                                     	mov    %rax,0x8(%rcx)
  1d4297:	8b 44 24 1c                                     	mov    0x1c(%rsp),%eax
  1d429b:	66 89 41 10                                     	mov    %ax,0x10(%rcx)
  1d429f:	48 8b 84 24 20 01 00 00                         	mov    0x120(%rsp),%rax
  1d42a7:	88 41 12                                        	mov    %al,0x12(%rcx)
  1d42aa:	c6 41 13 01                                     	movb   $0x1,0x13(%rcx)
  1d42ae:	c6 01 ff                                        	movb   $0xff,(%rcx)
  1d42b1:	e9 bc 00 00 00                                  	jmp    1d4372 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1652>
  1d42b6:	b9 2a 00 00 00                                  	mov    $0x2a,%ecx
  1d42bb:	48 8d 3d 46 e0 e4 ff                            	lea    -0x1b1fba(%rip),%rdi        # 22308 <anon.38d8fac72642096468ff551e1950d360.1255.llvm.10326835611459877753+0x4648>
  1d42c2:	e9 86 00 00 00                                  	jmp    1d434d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x162d>
  1d42c7:	8b 8c 24 b9 00 00 00                            	mov    0xb9(%rsp),%ecx
  1d42ce:	8b 94 24 bc 00 00 00                            	mov    0xbc(%rsp),%edx
  1d42d5:	89 94 24 eb 00 00 00                            	mov    %edx,0xeb(%rsp)
  1d42dc:	89 8c 24 e8 00 00 00                            	mov    %ecx,0xe8(%rsp)
  1d42e3:	eb 58                                           	jmp    1d433d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x161d>
  1d42e5:	8b 8c 24 b9 00 00 00                            	mov    0xb9(%rsp),%ecx
  1d42ec:	8b 94 24 bc 00 00 00                            	mov    0xbc(%rsp),%edx
  1d42f3:	89 94 24 f3 00 00 00                            	mov    %edx,0xf3(%rsp)
  1d42fa:	89 8c 24 f0 00 00 00                            	mov    %ecx,0xf0(%rsp)
  1d4301:	eb 3a                                           	jmp    1d433d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x161d>
  1d4303:	8b 8c 24 b9 00 00 00                            	mov    0xb9(%rsp),%ecx
  1d430a:	8b 94 24 bc 00 00 00                            	mov    0xbc(%rsp),%edx
  1d4311:	89 94 24 fb 00 00 00                            	mov    %edx,0xfb(%rsp)
  1d4318:	89 8c 24 f8 00 00 00                            	mov    %ecx,0xf8(%rsp)
  1d431f:	eb 1c                                           	jmp    1d433d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x161d>
  1d4321:	8b 8c 24 b9 00 00 00                            	mov    0xb9(%rsp),%ecx
  1d4328:	8b 94 24 bc 00 00 00                            	mov    0xbc(%rsp),%edx
  1d432f:	89 94 24 03 01 00 00                            	mov    %edx,0x103(%rsp)
  1d4336:	89 8c 24 00 01 00 00                            	mov    %ecx,0x100(%rsp)
  1d433d:	48 8b bc 24 c0 00 00 00                         	mov    0xc0(%rsp),%rdi
  1d4345:	48 8b 8c 24 c8 00 00 00                         	mov    0xc8(%rsp),%rcx
  1d434d:	48 8b b4 24 08 01 00 00                         	mov    0x108(%rsp),%rsi
  1d4355:	88 06                                           	mov    %al,(%rsi)
  1d4357:	48 8b 94 24 d8 00 00 00                         	mov    0xd8(%rsp),%rdx
  1d435f:	8b 02                                           	mov    (%rdx),%eax
  1d4361:	8b 52 03                                        	mov    0x3(%rdx),%edx
  1d4364:	89 46 01                                        	mov    %eax,0x1(%rsi)
  1d4367:	89 56 04                                        	mov    %edx,0x4(%rsi)
  1d436a:	48 89 7e 08                                     	mov    %rdi,0x8(%rsi)
  1d436e:	48 89 4e 10                                     	mov    %rcx,0x10(%rsi)
  1d4372:	48 81 c4 28 01 00 00                            	add    $0x128,%rsp
  1d4379:	5b                                              	pop    %rbx
  1d437a:	41 5c                                           	pop    %r12
  1d437c:	41 5d                                           	pop    %r13
  1d437e:	41 5e                                           	pop    %r14
  1d4380:	41 5f                                           	pop    %r15
  1d4382:	5d                                              	pop    %rbp
  1d4383:	c3                                              	ret
  1d4384:	48 89 fe                                        	mov    %rdi,%rsi
  1d4387:	48 8d 0d 22 cc 09 00                            	lea    0x9cc22(%rip),%rcx        # 270fb0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3338>
  1d438e:	4c 89 fa                                        	mov    %r15,%rdx
  1d4391:	ff 15 89 0c 0a 00                               	call   *0xa0c89(%rip)        # 275020 <_DYNAMIC+0x470>
  1d4397:	48 89 fe                                        	mov    %rdi,%rsi
  1d439a:	48 8d 0d 0f cc 09 00                            	lea    0x9cc0f(%rip),%rcx        # 270fb0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3338>
  1d43a1:	4c 89 f2                                        	mov    %r14,%rdx
  1d43a4:	ff 15 76 0c 0a 00                               	call   *0xa0c76(%rip)        # 275020 <_DYNAMIC+0x470>
  1d43aa:	48 89 fe                                        	mov    %rdi,%rsi
  1d43ad:	48 8d 0d fc cb 09 00                            	lea    0x9cbfc(%rip),%rcx        # 270fb0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3338>
  1d43b4:	4c 89 e2                                        	mov    %r12,%rdx
  1d43b7:	ff 15 63 0c 0a 00                               	call   *0xa0c63(%rip)        # 275020 <_DYNAMIC+0x470>
