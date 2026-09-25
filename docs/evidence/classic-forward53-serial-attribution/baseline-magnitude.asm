Disassembly of section .text:

00000000001b6b20 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>>:
  1b6b20:	55                                              	push   %rbp
  1b6b21:	41 57                                           	push   %r15
  1b6b23:	41 56                                           	push   %r14
  1b6b25:	41 55                                           	push   %r13
  1b6b27:	41 54                                           	push   %r12
  1b6b29:	53                                              	push   %rbx
  1b6b2a:	48 81 ec 98 00 00 00                            	sub    $0x98,%rsp
  1b6b31:	48 8b 47 40                                     	mov    0x40(%rdi),%rax
  1b6b35:	48 89 44 24 18                                  	mov    %rax,0x18(%rsp)
  1b6b3a:	48 85 c0                                        	test   %rax,%rax
  1b6b3d:	0f 84 6c 04 00 00                               	je     1b6faf <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x48f>
  1b6b43:	48 8b 47 30                                     	mov    0x30(%rdi),%rax
  1b6b47:	48 89 44 24 60                                  	mov    %rax,0x60(%rsp)
  1b6b4c:	48 85 c0                                        	test   %rax,%rax
  1b6b4f:	0f 84 5a 04 00 00                               	je     1b6faf <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x48f>
  1b6b55:	48 89 f3                                        	mov    %rsi,%rbx
  1b6b58:	0f b6 47 48                                     	movzbl 0x48(%rdi),%eax
  1b6b5c:	83 e0 1f                                        	and    $0x1f,%eax
  1b6b5f:	89 44 24 14                                     	mov    %eax,0x14(%rsp)
  1b6b63:	4c 8b 67 38                                     	mov    0x38(%rdi),%r12
  1b6b67:	48 8b 17                                        	mov    (%rdi),%rdx
  1b6b6a:	4c 8b 5f 08                                     	mov    0x8(%rdi),%r11
  1b6b6e:	48 8b 47 28                                     	mov    0x28(%rdi),%rax
  1b6b72:	48 89 44 24 20                                  	mov    %rax,0x20(%rsp)
  1b6b77:	48 8b 47 20                                     	mov    0x20(%rdi),%rax
  1b6b7b:	48 89 44 24 78                                  	mov    %rax,0x78(%rsp)
  1b6b80:	4c 8b 4e 08                                     	mov    0x8(%rsi),%r9
  1b6b84:	44 8b 46 18                                     	mov    0x18(%rsi),%r8d
  1b6b88:	44 8b 56 10                                     	mov    0x10(%rsi),%r10d
  1b6b8c:	8b 6e 14                                        	mov    0x14(%rsi),%ebp
  1b6b8f:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1b6b94:	48 89 ce                                        	mov    %rcx,%rsi
  1b6b97:	48 c1 ee 02                                     	shr    $0x2,%rsi
  1b6b9b:	89 c8                                           	mov    %ecx,%eax
  1b6b9d:	83 e0 03                                        	and    $0x3,%eax
  1b6ba0:	48 83 f8 01                                     	cmp    $0x1,%rax
  1b6ba4:	48 89 d0                                        	mov    %rdx,%rax
  1b6ba7:	48 83 de ff                                     	sbb    $0xffffffffffffffff,%rsi
  1b6bab:	31 d2                                           	xor    %edx,%edx
  1b6bad:	4c 89 4c 24 08                                  	mov    %r9,0x8(%rsp)
  1b6bb2:	4c 89 5c 24 38                                  	mov    %r11,0x38(%rsp)
  1b6bb7:	4c 89 64 24 30                                  	mov    %r12,0x30(%rsp)
  1b6bbc:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1b6bc1:	eb 36                                           	jmp    1b6bf9 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0xd9>
  1b6bc3:	66 66 66 66 2e 0f 1f 84 00 00 00 00 00          	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  1b6bd0:	4c 89 de                                        	mov    %r11,%rsi
  1b6bd3:	49 89 f3                                        	mov    %rsi,%r11
  1b6bd6:	48 8b 54 24 50                                  	mov    0x50(%rsp),%rdx
  1b6bdb:	48 83 c2 04                                     	add    $0x4,%rdx
  1b6bdf:	48 8b 74 24 58                                  	mov    0x58(%rsp),%rsi
  1b6be4:	48 ff ce                                        	dec    %rsi
  1b6be7:	48 8b 4c 24 48                                  	mov    0x48(%rsp),%rcx
  1b6bec:	48 83 c1 fc                                     	add    $0xfffffffffffffffc,%rcx
  1b6bf0:	48 85 f6                                        	test   %rsi,%rsi
  1b6bf3:	0f 84 b6 03 00 00                               	je     1b6faf <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x48f>
  1b6bf9:	48 89 74 24 58                                  	mov    %rsi,0x58(%rsp)
  1b6bfe:	48 83 f9 01                                     	cmp    $0x1,%rcx
  1b6c02:	48 89 4c 24 48                                  	mov    %rcx,0x48(%rsp)
  1b6c07:	48 89 ce                                        	mov    %rcx,%rsi
  1b6c0a:	48 83 d6 00                                     	adc    $0x0,%rsi
  1b6c0e:	48 83 fe 04                                     	cmp    $0x4,%rsi
  1b6c12:	b9 04 00 00 00                                  	mov    $0x4,%ecx
  1b6c17:	48 0f 43 f1                                     	cmovae %rcx,%rsi
  1b6c1b:	48 89 b4 24 90 00 00 00                         	mov    %rsi,0x90(%rsp)
  1b6c23:	48 39 54 24 18                                  	cmp    %rdx,0x18(%rsp)
  1b6c28:	48 89 54 24 50                                  	mov    %rdx,0x50(%rsp)
  1b6c2d:	74 a1                                           	je     1b6bd0 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0xb0>
  1b6c2f:	48 83 ca 01                                     	or     $0x1,%rdx
  1b6c33:	49 0f af d4                                     	imul   %r12,%rdx
  1b6c37:	48 ff c2                                        	inc    %rdx
  1b6c3a:	48 89 54 24 68                                  	mov    %rdx,0x68(%rsp)
  1b6c3f:	41 89 ed                                        	mov    %ebp,%r13d
  1b6c42:	31 d2                                           	xor    %edx,%edx
  1b6c44:	4c 89 de                                        	mov    %r11,%rsi
  1b6c47:	eb 1a                                           	jmp    1b6c63 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x143>
  1b6c49:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1b6c50:	49 89 cc                                        	mov    %rcx,%r12
  1b6c53:	48 8b 54 24 70                                  	mov    0x70(%rsp),%rdx
  1b6c58:	48 3b 54 24 60                                  	cmp    0x60(%rsp),%rdx
  1b6c5d:	0f 84 70 ff ff ff                               	je     1b6bd3 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0xb3>
  1b6c63:	48 8b 4c 24 68                                  	mov    0x68(%rsp),%rcx
  1b6c68:	4c 8d 34 11                                     	lea    (%rcx,%rdx,1),%r14
  1b6c6c:	48 ff c2                                        	inc    %rdx
  1b6c6f:	48 89 54 24 70                                  	mov    %rdx,0x70(%rsp)
  1b6c74:	45 89 ef                                        	mov    %r13d,%r15d
  1b6c77:	45 31 db                                        	xor    %r11d,%r11d
  1b6c7a:	4c 89 e1                                        	mov    %r12,%rcx
  1b6c7d:	eb 3b                                           	jmp    1b6cba <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x19a>
  1b6c7f:	45 89 e2                                        	mov    %r12d,%r10d
  1b6c82:	48 8b 84 24 80 00 00 00                         	mov    0x80(%rsp),%rax
  1b6c8a:	c6 40 02 01                                     	movb   $0x1,0x2(%rax)
  1b6c8e:	48 8b 74 24 38                                  	mov    0x38(%rsp),%rsi
  1b6c93:	48 8b 4c 24 30                                  	mov    0x30(%rsp),%rcx
  1b6c98:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1b6c9d:	4c 8b 74 24 40                                  	mov    0x40(%rsp),%r14
  1b6ca2:	4c 8b 9c 24 88 00 00 00                         	mov    0x88(%rsp),%r11
  1b6caa:	49 ff c3                                        	inc    %r11
  1b6cad:	49 01 ce                                        	add    %rcx,%r14
  1b6cb0:	4c 3b 9c 24 90 00 00 00                         	cmp    0x90(%rsp),%r11
  1b6cb8:	74 96                                           	je     1b6c50 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x130>
  1b6cba:	49 39 f6                                        	cmp    %rsi,%r14
  1b6cbd:	0f 83 fe 02 00 00                               	jae    1b6fc1 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x4a1>
  1b6cc3:	4f 8d 24 76                                     	lea    (%r14,%r14,2),%r12
  1b6cc7:	42 80 3c 20 01                                  	cmpb   $0x1,(%rax,%r12,1)
  1b6ccc:	75 dc                                           	jne    1b6caa <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x18a>
  1b6cce:	49 01 c4                                        	add    %rax,%r12
  1b6cd1:	41 80 7c 24 01 00                               	cmpb   $0x0,0x1(%r12)
  1b6cd7:	75 d1                                           	jne    1b6caa <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x18a>
  1b6cd9:	b8 10 00 00 00                                  	mov    $0x10,%eax
  1b6cde:	41 80 7c 24 02 00                               	cmpb   $0x0,0x2(%r12)
  1b6ce4:	4c 89 74 24 40                                  	mov    %r14,0x40(%rsp)
  1b6ce9:	4c 89 9c 24 88 00 00 00                         	mov    %r11,0x88(%rsp)
  1b6cf1:	4c 89 a4 24 80 00 00 00                         	mov    %r12,0x80(%rsp)
  1b6cf9:	75 79                                           	jne    1b6d74 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x254>
  1b6cfb:	48 8b 7c 24 28                                  	mov    0x28(%rsp),%rdi
  1b6d00:	48 8b 74 24 38                                  	mov    0x38(%rsp),%rsi
  1b6d05:	4c 89 f2                                        	mov    %r14,%rdx
  1b6d08:	48 8b 4c 24 30                                  	mov    0x30(%rsp),%rcx
  1b6d0d:	45 89 d6                                        	mov    %r10d,%r14d
  1b6d10:	45 89 c4                                        	mov    %r8d,%r12d
  1b6d13:	e8 b8 85 ff ff                                  	call   1af2d0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1b6d18:	45 89 e0                                        	mov    %r12d,%r8d
  1b6d1b:	45 89 f2                                        	mov    %r14d,%r10d
  1b6d1e:	4c 8b 74 24 40                                  	mov    0x40(%rsp),%r14
  1b6d23:	4c 8b 4c 24 08                                  	mov    0x8(%rsp),%r9
  1b6d28:	89 c1                                           	mov    %eax,%ecx
  1b6d2a:	c1 e9 18                                        	shr    $0x18,%ecx
  1b6d2d:	80 e1 01                                        	and    $0x1,%cl
  1b6d30:	48 89 c2                                        	mov    %rax,%rdx
  1b6d33:	48 c1 ea 20                                     	shr    $0x20,%rdx
  1b6d37:	80 e2 01                                        	and    $0x1,%dl
  1b6d3a:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1b6d3f:	10 ca                                           	adc    %cl,%dl
  1b6d41:	0f ba e0 08                                     	bt     $0x8,%eax
  1b6d45:	80 d2 00                                        	adc    $0x0,%dl
  1b6d48:	48 89 c1                                        	mov    %rax,%rcx
  1b6d4b:	48 c1 e9 38                                     	shr    $0x38,%rcx
  1b6d4f:	89 c6                                           	mov    %eax,%esi
  1b6d51:	40 80 e6 01                                     	and    $0x1,%sil
  1b6d55:	89 c7                                           	mov    %eax,%edi
  1b6d57:	c1 ef 10                                        	shr    $0x10,%edi
  1b6d5a:	40 80 e7 01                                     	and    $0x1,%dil
  1b6d5e:	40 00 f1                                        	add    %sil,%cl
  1b6d61:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1b6d66:	40 10 f9                                        	adc    %dil,%cl
  1b6d69:	31 c0                                           	xor    %eax,%eax
  1b6d6b:	08 d1                                           	or     %dl,%cl
  1b6d6d:	0f 94 c0                                        	sete   %al
  1b6d70:	48 83 f0 0f                                     	xor    $0xf,%rax
  1b6d74:	4c 3b 74 24 20                                  	cmp    0x20(%rsp),%r14
  1b6d79:	0f 83 52 02 00 00                               	jae    1b6fd1 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x4b1>
  1b6d7f:	0f b6 7c 43 1c                                  	movzbl 0x1c(%rbx,%rax,2),%edi
  1b6d84:	48 83 ff 2e                                     	cmp    $0x2e,%rdi
  1b6d88:	0f 87 58 02 00 00                               	ja     1b6fe6 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x4c6>
  1b6d8e:	45 89 c3                                        	mov    %r8d,%r11d
  1b6d91:	48 8b 4c 24 78                                  	mov    0x78(%rsp),%rcx
  1b6d96:	42 8b 0c b1                                     	mov    (%rcx,%r14,4),%ecx
  1b6d9a:	45 31 c0                                        	xor    %r8d,%r8d
  1b6d9d:	8b 54 24 14                                     	mov    0x14(%rsp),%edx
  1b6da1:	0f a3 d1                                        	bt     %edx,%ecx
  1b6da4:	41 0f 92 c0                                     	setb   %r8b
  1b6da8:	0f b6 4c 43 1d                                  	movzbl 0x1d(%rbx,%rax,2),%ecx
  1b6dad:	48 8d 15 bc b7 e6 ff                            	lea    -0x194844(%rip),%rdx        # 22570 <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929+0x31>
  1b6db4:	45 89 d6                                        	mov    %r10d,%r14d
  1b6db7:	49 89 d2                                        	mov    %rdx,%r10
  1b6dba:	44 8b 24 fa                                     	mov    (%rdx,%rdi,8),%r12d
  1b6dbe:	0f b6 54 fa 04                                  	movzbl 0x4(%rdx,%rdi,8),%edx
  1b6dc3:	41 0f b6 74 fa 05                               	movzbl 0x5(%r10,%rdi,8),%esi
  1b6dc9:	41 0f b6 7c fa 06                               	movzbl 0x6(%r10,%rdi,8),%edi
  1b6dcf:	45 89 f2                                        	mov    %r14d,%r10d
  1b6dd2:	45 29 e2                                        	sub    %r12d,%r10d
  1b6dd5:	44 89 53 10                                     	mov    %r10d,0x10(%rbx)
  1b6dd9:	41 39 c8                                        	cmp    %ecx,%r8d
  1b6ddc:	75 17                                           	jne    1b6df5 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x2d5>
  1b6dde:	66 45 85 d2                                     	test   %r10w,%r10w
  1b6de2:	45 89 d8                                        	mov    %r11d,%r8d
  1b6de5:	78 35                                           	js     1b6e1c <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x2fc>
  1b6de7:	45 39 e2                                        	cmp    %r12d,%r10d
  1b6dea:	73 66                                           	jae    1b6e52 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x332>
  1b6dec:	44 89 63 10                                     	mov    %r12d,0x10(%rbx)
  1b6df0:	45 89 e2                                        	mov    %r12d,%r10d
  1b6df3:	eb 6d                                           	jmp    1b6e62 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x342>
  1b6df5:	45 39 e2                                        	cmp    %r12d,%r10d
  1b6df8:	45 89 d8                                        	mov    %r11d,%r8d
  1b6dfb:	73 34                                           	jae    1b6e31 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x311>
  1b6dfd:	45 01 fc                                        	add    %r15d,%r12d
  1b6e00:	44 89 63 14                                     	mov    %r12d,0x14(%rbx)
  1b6e04:	44 89 e5                                        	mov    %r12d,%ebp
  1b6e07:	45 89 e5                                        	mov    %r12d,%r13d
  1b6e0a:	45 89 e7                                        	mov    %r12d,%r15d
  1b6e0d:	45 89 d4                                        	mov    %r10d,%r12d
  1b6e10:	40 88 74 43 1c                                  	mov    %sil,0x1c(%rbx,%rax,2)
  1b6e15:	40 84 ff                                        	test   %dil,%dil
  1b6e18:	75 25                                           	jne    1b6e3f <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x31f>
  1b6e1a:	eb 2a                                           	jmp    1b6e46 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x326>
  1b6e1c:	45 01 fc                                        	add    %r15d,%r12d
  1b6e1f:	44 89 63 14                                     	mov    %r12d,0x14(%rbx)
  1b6e23:	44 89 e5                                        	mov    %r12d,%ebp
  1b6e26:	45 89 e5                                        	mov    %r12d,%r13d
  1b6e29:	45 89 e7                                        	mov    %r12d,%r15d
  1b6e2c:	e9 51 fe ff ff                                  	jmp    1b6c82 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x162>
  1b6e31:	44 89 63 10                                     	mov    %r12d,0x10(%rbx)
  1b6e35:	40 88 74 43 1c                                  	mov    %sil,0x1c(%rbx,%rax,2)
  1b6e3a:	40 84 ff                                        	test   %dil,%dil
  1b6e3d:	74 07                                           	je     1b6e46 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x326>
  1b6e3f:	80 f1 01                                        	xor    $0x1,%cl
  1b6e42:	88 4c 43 1d                                     	mov    %cl,0x1d(%rbx,%rax,2)
  1b6e46:	66 45 85 e4                                     	test   %r12w,%r12w
  1b6e4a:	0f 88 2f fe ff ff                               	js     1b6c7f <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x15f>
  1b6e50:	eb 21                                           	jmp    1b6e73 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x353>
  1b6e52:	45 01 fc                                        	add    %r15d,%r12d
  1b6e55:	44 89 63 14                                     	mov    %r12d,0x14(%rbx)
  1b6e59:	44 89 e5                                        	mov    %r12d,%ebp
  1b6e5c:	45 89 e5                                        	mov    %r12d,%r13d
  1b6e5f:	45 89 e7                                        	mov    %r12d,%r15d
  1b6e62:	88 54 43 1c                                     	mov    %dl,0x1c(%rbx,%rax,2)
  1b6e66:	45 89 d4                                        	mov    %r10d,%r12d
  1b6e69:	66 45 85 e4                                     	test   %r12w,%r12w
  1b6e6d:	0f 88 0c fe ff ff                               	js     1b6c7f <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x15f>
  1b6e73:	0f b6 43 45                                     	movzbl 0x45(%rbx),%eax
  1b6e77:	44 0f b6 73 46                                  	movzbl 0x46(%rbx),%r14d
  1b6e7c:	44 89 fd                                        	mov    %r15d,%ebp
  1b6e7f:	eb 4a                                           	jmp    1b6ecb <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x3ab>
  1b6e81:	49 8b 41 08                                     	mov    0x8(%r9),%rax
  1b6e85:	46 88 34 38                                     	mov    %r14b,(%rax,%r15,1)
  1b6e89:	49 ff c7                                        	inc    %r15
  1b6e8c:	4d 89 79 10                                     	mov    %r15,0x10(%r9)
  1b6e90:	41 89 ee                                        	mov    %ebp,%r14d
  1b6e93:	41 c1 ee 13                                     	shr    $0x13,%r14d
  1b6e97:	41 b8 08 00 00 00                               	mov    $0x8,%r8d
  1b6e9d:	b8 fe ff 07 00                                  	mov    $0x7fffe,%eax
  1b6ea2:	c6 43 45 01                                     	movb   $0x1,0x45(%rbx)
  1b6ea6:	44 88 73 46                                     	mov    %r14b,0x46(%rbx)
  1b6eaa:	21 c5                                           	and    %eax,%ebp
  1b6eac:	89 6b 14                                        	mov    %ebp,0x14(%rbx)
  1b6eaf:	44 89 43 18                                     	mov    %r8d,0x18(%rbx)
  1b6eb3:	b0 01                                           	mov    $0x1,%al
  1b6eb5:	41 89 ed                                        	mov    %ebp,%r13d
  1b6eb8:	41 89 ef                                        	mov    %ebp,%r15d
  1b6ebb:	41 f7 c4 00 40 00 00                            	test   $0x4000,%r12d
  1b6ec2:	45 89 d4                                        	mov    %r10d,%r12d
  1b6ec5:	0f 85 b7 fd ff ff                               	jne    1b6c82 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x162>
  1b6ecb:	47 8d 14 24                                     	lea    (%r12,%r12,1),%r10d
  1b6ecf:	44 89 53 10                                     	mov    %r10d,0x10(%rbx)
  1b6ed3:	01 ed                                           	add    %ebp,%ebp
  1b6ed5:	89 6b 14                                        	mov    %ebp,0x14(%rbx)
  1b6ed8:	41 ff c8                                        	dec    %r8d
  1b6edb:	44 89 43 18                                     	mov    %r8d,0x18(%rbx)
  1b6edf:	75 d4                                           	jne    1b6eb5 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x395>
  1b6ee1:	c6 43 45 00                                     	movb   $0x0,0x45(%rbx)
  1b6ee5:	3c 01                                           	cmp    $0x1,%al
  1b6ee7:	75 a7                                           	jne    1b6e90 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x370>
  1b6ee9:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1b6eed:	74 56                                           	je     1b6f45 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x425>
  1b6eef:	81 fd ff ff ff 07                               	cmp    $0x7ffffff,%ebp
  1b6ef5:	0f 86 8e 00 00 00                               	jbe    1b6f89 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x469>
  1b6efb:	41 fe c6                                        	inc    %r14b
  1b6efe:	4d 8b 79 10                                     	mov    0x10(%r9),%r15
  1b6f02:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1b6f06:	0f 85 81 00 00 00                               	jne    1b6f8d <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x46d>
  1b6f0c:	41 89 ee                                        	mov    %ebp,%r14d
  1b6f0f:	41 81 e6 fe ff ff 07                            	and    $0x7fffffe,%r14d
  1b6f16:	44 89 73 14                                     	mov    %r14d,0x14(%rbx)
  1b6f1a:	4d 3b 39                                        	cmp    (%r9),%r15
  1b6f1d:	75 14                                           	jne    1b6f33 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x413>
  1b6f1f:	4c 89 cf                                        	mov    %r9,%rdi
  1b6f22:	45 89 d5                                        	mov    %r10d,%r13d
  1b6f25:	ff 15 c5 6e 0b 00                               	call   *0xb6ec5(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b6f2b:	45 89 ea                                        	mov    %r13d,%r10d
  1b6f2e:	4c 8b 4c 24 08                                  	mov    0x8(%rsp),%r9
  1b6f33:	49 8b 41 08                                     	mov    0x8(%r9),%rax
  1b6f37:	42 c6 04 38 ff                                  	movb   $0xff,(%rax,%r15,1)
  1b6f3c:	49 ff c7                                        	inc    %r15
  1b6f3f:	4d 89 79 10                                     	mov    %r15,0x10(%r9)
  1b6f43:	eb 30                                           	jmp    1b6f75 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x455>
  1b6f45:	4d 8b 71 10                                     	mov    0x10(%r9),%r14
  1b6f49:	4d 3b 31                                        	cmp    (%r9),%r14
  1b6f4c:	75 14                                           	jne    1b6f62 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x442>
  1b6f4e:	4c 89 cf                                        	mov    %r9,%rdi
  1b6f51:	45 89 d7                                        	mov    %r10d,%r15d
  1b6f54:	ff 15 96 6e 0b 00                               	call   *0xb6e96(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b6f5a:	45 89 fa                                        	mov    %r15d,%r10d
  1b6f5d:	4c 8b 4c 24 08                                  	mov    0x8(%rsp),%r9
  1b6f62:	49 8b 41 08                                     	mov    0x8(%r9),%rax
  1b6f66:	42 c6 04 30 ff                                  	movb   $0xff,(%rax,%r14,1)
  1b6f6b:	49 ff c6                                        	inc    %r14
  1b6f6e:	4d 89 71 10                                     	mov    %r14,0x10(%r9)
  1b6f72:	41 89 ee                                        	mov    %ebp,%r14d
  1b6f75:	41 c1 ee 14                                     	shr    $0x14,%r14d
  1b6f79:	41 b8 07 00 00 00                               	mov    $0x7,%r8d
  1b6f7f:	b8 fe ff 0f 00                                  	mov    $0xffffe,%eax
  1b6f84:	e9 19 ff ff ff                                  	jmp    1b6ea2 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x382>
  1b6f89:	4d 8b 79 10                                     	mov    0x10(%r9),%r15
  1b6f8d:	4d 3b 39                                        	cmp    (%r9),%r15
  1b6f90:	0f 85 eb fe ff ff                               	jne    1b6e81 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x361>
  1b6f96:	4c 89 cf                                        	mov    %r9,%rdi
  1b6f99:	45 89 d5                                        	mov    %r10d,%r13d
  1b6f9c:	ff 15 4e 6e 0b 00                               	call   *0xb6e4e(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1b6fa2:	45 89 ea                                        	mov    %r13d,%r10d
  1b6fa5:	4c 8b 4c 24 08                                  	mov    0x8(%rsp),%r9
  1b6faa:	e9 d2 fe ff ff                                  	jmp    1b6e81 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>+0x361>
  1b6faf:	48 81 c4 98 00 00 00                            	add    $0x98,%rsp
  1b6fb6:	5b                                              	pop    %rbx
  1b6fb7:	41 5c                                           	pop    %r12
  1b6fb9:	41 5d                                           	pop    %r13
  1b6fbb:	41 5e                                           	pop    %r14
  1b6fbd:	41 5f                                           	pop    %r15
  1b6fbf:	5d                                              	pop    %rbp
  1b6fc0:	c3                                              	ret
  1b6fc1:	48 8d 15 68 28 0b 00                            	lea    0xb2868(%rip),%rdx        # 269830 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2888>
  1b6fc8:	4c 89 f7                                        	mov    %r14,%rdi
  1b6fcb:	ff 15 d7 6d 0b 00                               	call   *0xb6dd7(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b6fd1:	48 8d 15 e0 27 0b 00                            	lea    0xb27e0(%rip),%rdx        # 2697b8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x2810>
  1b6fd8:	4c 89 f7                                        	mov    %r14,%rdi
  1b6fdb:	48 8b 74 24 20                                  	mov    0x20(%rsp),%rsi
  1b6fe0:	ff 15 c2 6d 0b 00                               	call   *0xb6dc2(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1b6fe6:	48 8d 15 0b 30 0b 00                            	lea    0xb300b(%rip),%rdx        # 269ff8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3050>
  1b6fed:	be 2f 00 00 00                                  	mov    $0x2f,%esi
  1b6ff2:	ff 15 b0 6d 0b 00                               	call   *0xb6db0(%rip)        # 26dda8 <_DYNAMIC+0x228>
