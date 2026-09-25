Disassembly of section .text:

00000000001d4b70 <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch>:
  1d4b70:	55                                              	push   %rbp
  1d4b71:	41 57                                           	push   %r15
  1d4b73:	41 56                                           	push   %r14
  1d4b75:	41 55                                           	push   %r13
  1d4b77:	41 54                                           	push   %r12
  1d4b79:	53                                              	push   %rbx
  1d4b7a:	48 81 ec b8 00 00 00                            	sub    $0xb8,%rsp
  1d4b81:	4d 89 c2                                        	mov    %r8,%r10
  1d4b84:	49 c1 ea 28                                     	shr    $0x28,%r10
  1d4b88:	44 89 d0                                        	mov    %r10d,%eax
  1d4b8b:	41 b3 02                                        	mov    $0x2,%r11b
  1d4b8e:	24 c0                                           	and    $0xc0,%al
  1d4b90:	74 32                                           	je     1d4bc4 <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x54>
  1d4b92:	44 88 1f                                        	mov    %r11b,(%rdi)
  1d4b95:	44 88 57 01                                     	mov    %r10b,0x1(%rdi)
  1d4b99:	88 47 02                                        	mov    %al,0x2(%rdi)
  1d4b9c:	48 8d 05 61 d9 e4 ff                            	lea    -0x1b269f(%rip),%rax        # 22504 <anon.c3db339937c4b26e029c5c277d8a0513.100.llvm.11782617808337995929>
  1d4ba3:	48 89 47 08                                     	mov    %rax,0x8(%rdi)
  1d4ba7:	48 c7 47 10 3b 00 00 00                         	movq   $0x3b,0x10(%rdi)
  1d4baf:	48 89 f8                                        	mov    %rdi,%rax
  1d4bb2:	48 81 c4 b8 00 00 00                            	add    $0xb8,%rsp
  1d4bb9:	5b                                              	pop    %rbx
  1d4bba:	41 5c                                           	pop    %r12
  1d4bbc:	41 5d                                           	pop    %r13
  1d4bbe:	41 5e                                           	pop    %r14
  1d4bc0:	41 5f                                           	pop    %r15
  1d4bc2:	5d                                              	pop    %rbp
  1d4bc3:	c3                                              	ret
  1d4bc4:	4d 89 c4                                        	mov    %r8,%r12
  1d4bc7:	49 c1 ec 20                                     	shr    $0x20,%r12
  1d4bcb:	45 84 e4                                        	test   %r12b,%r12b
  1d4bce:	74 16                                           	je     1d4be6 <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x76>
  1d4bd0:	45 0f b7 f0                                     	movzwl %r8w,%r14d
  1d4bd4:	4c 39 f1                                        	cmp    %r14,%rcx
  1d4bd7:	73 12                                           	jae    1d4beb <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x7b>
  1d4bd9:	c6 07 01                                        	movb   $0x1,(%rdi)
  1d4bdc:	4c 89 77 08                                     	mov    %r14,0x8(%rdi)
  1d4be0:	48 89 4f 10                                     	mov    %rcx,0x10(%rdi)
  1d4be4:	eb c9                                           	jmp    1d4baf <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x3f>
  1d4be6:	41 b3 03                                        	mov    $0x3,%r11b
  1d4be9:	eb a7                                           	jmp    1d4b92 <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x22>
  1d4beb:	49 89 d7                                        	mov    %rdx,%r15
  1d4bee:	4d 89 c3                                        	mov    %r8,%r11
  1d4bf1:	49 c1 eb 10                                     	shr    $0x10,%r11
  1d4bf5:	41 0f b7 db                                     	movzwl %r11w,%ebx
  1d4bf9:	48 8d 43 ff                                     	lea    -0x1(%rbx),%rax
  1d4bfd:	48 f7 e1                                        	mul    %rcx
  1d4c00:	0f 80 49 01 00 00                               	jo     1d4d4f <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x1df>
  1d4c06:	4c 01 f0                                        	add    %r14,%rax
  1d4c09:	0f 82 40 01 00 00                               	jb     1d4d4f <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x1df>
  1d4c0f:	4c 89 fa                                        	mov    %r15,%rdx
  1d4c12:	49 39 c7                                        	cmp    %rax,%r15
  1d4c15:	73 0d                                           	jae    1d4c24 <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0xb4>
  1d4c17:	c6 07 01                                        	movb   $0x1,(%rdi)
  1d4c1a:	48 89 47 08                                     	mov    %rax,0x8(%rdi)
  1d4c1e:	48 89 57 10                                     	mov    %rdx,0x10(%rdi)
  1d4c22:	eb 8b                                           	jmp    1d4baf <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x3f>
  1d4c24:	48 8b 84 24 f0 00 00 00                         	mov    0xf0(%rsp),%rax
  1d4c2c:	41 80 fa 02                                     	cmp    $0x2,%r10b
  1d4c30:	73 4a                                           	jae    1d4c7c <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x10c>
  1d4c32:	66 41 83 f8 41                                  	cmp    $0x41,%r8w
  1d4c37:	41 0f 92 c2                                     	setb   %r10b
  1d4c3b:	66 41 83 fb 41                                  	cmp    $0x41,%r11w
  1d4c40:	41 0f 92 c3                                     	setb   %r11b
  1d4c44:	45 84 da                                        	test   %r11b,%r10b
  1d4c47:	74 33                                           	je     1d4c7c <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x10c>
  1d4c49:	49 ba 00 00 00 00 ff ff ff ff                   	movabs $0xffffffff00000000,%r10
  1d4c53:	4d 21 d0                                        	and    %r10,%r8
  1d4c56:	c1 e3 10                                        	shl    $0x10,%ebx
  1d4c59:	4c 09 c3                                        	or     %r8,%rbx
  1d4c5c:	4c 09 f3                                        	or     %r14,%rbx
  1d4c5f:	49 89 fe                                        	mov    %rdi,%r14
  1d4c62:	45 31 c0                                        	xor    %r8d,%r8d
  1d4c65:	50                                              	push   %rax
  1d4c66:	6a 00                                           	push   $0x0
  1d4c68:	41 51                                           	push   %r9
  1d4c6a:	53                                              	push   %rbx
  1d4c6b:	e8 e0 78 00 00                                  	call   1dc550 <emuella_j2k_tier1::packed_encode::encode>
  1d4c70:	48 83 c4 20                                     	add    $0x20,%rsp
  1d4c74:	4c 89 f0                                        	mov    %r14,%rax
  1d4c77:	e9 36 ff ff ff                                  	jmp    1d4bb2 <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x42>
  1d4c7c:	4d 89 cf                                        	mov    %r9,%r15
  1d4c7f:	49 89 fd                                        	mov    %rdi,%r13
  1d4c82:	4c 89 c5                                        	mov    %r8,%rbp
  1d4c85:	4c 89 c7                                        	mov    %r8,%rdi
  1d4c88:	48 c1 ef 30                                     	shr    $0x30,%rdi
  1d4c8c:	44 0f b6 c7                                     	movzbl %dil,%r8d
  1d4c90:	48 8d 7c 24 08                                  	lea    0x8(%rsp),%rdi
  1d4c95:	49 89 f1                                        	mov    %rsi,%r9
  1d4c98:	48 89 c6                                        	mov    %rax,%rsi
  1d4c9b:	48 89 d0                                        	mov    %rdx,%rax
  1d4c9e:	4c 89 f2                                        	mov    %r14,%rdx
  1d4ca1:	49 89 ca                                        	mov    %rcx,%r10
  1d4ca4:	48 89 d9                                        	mov    %rbx,%rcx
  1d4ca7:	41 52                                           	push   %r10
  1d4ca9:	50                                              	push   %rax
  1d4caa:	e8 21 5e 00 00                                  	call   1daad0 <<emuella_j2k_tier1::CodeBlockEncodeScratch>::prepare_strided_with_max>
  1d4caf:	48 83 c4 10                                     	add    $0x10,%rsp
  1d4cb3:	0f 10 44 24 48                                  	movups 0x48(%rsp),%xmm0
  1d4cb8:	0f 29 84 24 a0 00 00 00                         	movaps %xmm0,0xa0(%rsp)
  1d4cc0:	0f 10 44 24 08                                  	movups 0x8(%rsp),%xmm0
  1d4cc5:	0f 10 4c 24 18                                  	movups 0x18(%rsp),%xmm1
  1d4cca:	0f 10 54 24 28                                  	movups 0x28(%rsp),%xmm2
  1d4ccf:	0f 10 5c 24 38                                  	movups 0x38(%rsp),%xmm3
  1d4cd4:	0f 29 9c 24 90 00 00 00                         	movaps %xmm3,0x90(%rsp)
  1d4cdc:	0f 29 94 24 80 00 00 00                         	movaps %xmm2,0x80(%rsp)
  1d4ce4:	0f 29 4c 24 70                                  	movaps %xmm1,0x70(%rsp)
  1d4ce9:	0f 29 44 24 60                                  	movaps %xmm0,0x60(%rsp)
  1d4cee:	8b 54 24 58                                     	mov    0x58(%rsp),%edx
  1d4cf2:	85 d2                                           	test   %edx,%edx
  1d4cf4:	74 34                                           	je     1d4d2a <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x1ba>
  1d4cf6:	48 b8 00 00 00 00 ff ff ff ff                   	movabs $0xffffffff00000000,%rax
  1d4d00:	48 21 c5                                        	and    %rax,%rbp
  1d4d03:	c1 e3 10                                        	shl    $0x10,%ebx
  1d4d06:	48 09 eb                                        	or     %rbp,%rbx
  1d4d09:	4c 09 f3                                        	or     %r14,%rbx
  1d4d0c:	48 8d 74 24 60                                  	lea    0x60(%rsp),%rsi
  1d4d11:	4c 89 ef                                        	mov    %r13,%rdi
  1d4d14:	48 89 d9                                        	mov    %rbx,%rcx
  1d4d17:	4d 89 f8                                        	mov    %r15,%r8
  1d4d1a:	45 31 c9                                        	xor    %r9d,%r9d
  1d4d1d:	e8 fe c5 ff ff                                  	call   1d1320 <emuella_j2k_tier1::encode_prepared_baseline_code_block>
  1d4d22:	4c 89 ef                                        	mov    %r13,%rdi
  1d4d25:	e9 85 fe ff ff                                  	jmp    1d4baf <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x3f>
  1d4d2a:	4c 89 ef                                        	mov    %r13,%rdi
  1d4d2d:	49 c7 45 08 00 00 00 00                         	movq   $0x0,0x8(%r13)
  1d4d35:	66 41 c7 45 10 00 00                            	movw   $0x0,0x10(%r13)
  1d4d3c:	45 88 65 12                                     	mov    %r12b,0x12(%r13)
  1d4d40:	41 c6 45 13 00                                  	movb   $0x0,0x13(%r13)
  1d4d45:	41 c6 45 00 ff                                  	movb   $0xff,0x0(%r13)
  1d4d4a:	e9 60 fe ff ff                                  	jmp    1d4baf <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x3f>
  1d4d4f:	c6 07 03                                        	movb   $0x3,(%rdi)
  1d4d52:	48 8d 05 e6 d7 e4 ff                            	lea    -0x1b281a(%rip),%rax        # 2253f <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929>
  1d4d59:	48 89 47 08                                     	mov    %rax,0x8(%rdi)
  1d4d5d:	48 c7 47 10 30 00 00 00                         	movq   $0x30,0x10(%rdi)
  1d4d65:	e9 45 fe ff ff                                  	jmp    1d4baf <emuella_j2k_tier1::encode_baseline_code_block_with_strided_scratch+0x3f>
