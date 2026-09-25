Disassembly of section .text:

00000000001d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>:
  1d9c00:	53                                              	push   %rbx
  1d9c01:	48 89 fb                                        	mov    %rdi,%rbx
  1d9c04:	40 0f b6 fe                                     	movzbl %sil,%edi
  1d9c08:	40 80 ff 12                                     	cmp    $0x12,%dil
  1d9c0c:	0f 87 ad 00 00 00                               	ja     1d9cbf <<emuella_j2k_tier1::mq::Encoder>::write_bit+0xbf>
  1d9c12:	0f b6 44 7b 1c                                  	movzbl 0x1c(%rbx,%rdi,2),%eax
  1d9c17:	48 83 f8 2f                                     	cmp    $0x2f,%rax
  1d9c1b:	0f 83 b0 00 00 00                               	jae    1d9cd1 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0xd1>
  1d9c21:	0f b6 74 7b 1d                                  	movzbl 0x1d(%rbx,%rdi,2),%esi
  1d9c26:	4c 8d 15 43 89 e4 ff                            	lea    -0x1b76bd(%rip),%r10        # 22570 <anon.c3db339937c4b26e029c5c277d8a0513.101.llvm.11782617808337995929+0x31>
  1d9c2d:	41 8b 0c c2                                     	mov    (%r10,%rax,8),%ecx
  1d9c31:	45 0f b6 44 c2 04                               	movzbl 0x4(%r10,%rax,8),%r8d
  1d9c37:	45 0f b6 4c c2 05                               	movzbl 0x5(%r10,%rax,8),%r9d
  1d9c3d:	45 0f b6 54 c2 06                               	movzbl 0x6(%r10,%rax,8),%r10d
  1d9c43:	8b 43 10                                        	mov    0x10(%rbx),%eax
  1d9c46:	29 c8                                           	sub    %ecx,%eax
  1d9c48:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1d9c4b:	39 f2                                           	cmp    %esi,%edx
  1d9c4d:	75 13                                           	jne    1d9c62 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0x62>
  1d9c4f:	66 85 c0                                        	test   %ax,%ax
  1d9c52:	78 23                                           	js     1d9c77 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0x77>
  1d9c54:	39 c8                                           	cmp    %ecx,%eax
  1d9c56:	73 3c                                           	jae    1d9c94 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0x94>
  1d9c58:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1d9c5b:	44 88 44 7b 1c                                  	mov    %r8b,0x1c(%rbx,%rdi,2)
  1d9c60:	eb 56                                           	jmp    1d9cb8 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0xb8>
  1d9c62:	39 c8                                           	cmp    %ecx,%eax
  1d9c64:	73 16                                           	jae    1d9c7c <<emuella_j2k_tier1::mq::Encoder>::write_bit+0x7c>
  1d9c66:	01 4b 14                                        	add    %ecx,0x14(%rbx)
  1d9c69:	89 c1                                           	mov    %eax,%ecx
  1d9c6b:	44 88 4c 7b 1c                                  	mov    %r9b,0x1c(%rbx,%rdi,2)
  1d9c70:	45 84 d2                                        	test   %r10b,%r10b
  1d9c73:	74 43                                           	je     1d9cb8 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0xb8>
  1d9c75:	eb 12                                           	jmp    1d9c89 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0x89>
  1d9c77:	01 4b 14                                        	add    %ecx,0x14(%rbx)
  1d9c7a:	5b                                              	pop    %rbx
  1d9c7b:	c3                                              	ret
  1d9c7c:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1d9c7f:	44 88 4c 7b 1c                                  	mov    %r9b,0x1c(%rbx,%rdi,2)
  1d9c84:	45 84 d2                                        	test   %r10b,%r10b
  1d9c87:	74 2f                                           	je     1d9cb8 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0xb8>
  1d9c89:	40 80 f6 01                                     	xor    $0x1,%sil
  1d9c8d:	40 88 74 7b 1d                                  	mov    %sil,0x1d(%rbx,%rdi,2)
  1d9c92:	eb 24                                           	jmp    1d9cb8 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0xb8>
  1d9c94:	01 4b 14                                        	add    %ecx,0x14(%rbx)
  1d9c97:	44 88 44 7b 1c                                  	mov    %r8b,0x1c(%rbx,%rdi,2)
  1d9c9c:	89 c1                                           	mov    %eax,%ecx
  1d9c9e:	eb 18                                           	jmp    1d9cb8 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0xb8>
  1d9ca0:	01 c9                                           	add    %ecx,%ecx
  1d9ca2:	89 4b 10                                        	mov    %ecx,0x10(%rbx)
  1d9ca5:	d1 63 14                                        	shll   $1,0x14(%rbx)
  1d9ca8:	ff 4b 18                                        	decl   0x18(%rbx)
  1d9cab:	75 0b                                           	jne    1d9cb8 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0xb8>
  1d9cad:	48 89 df                                        	mov    %rbx,%rdi
  1d9cb0:	e8 1b fe ff ff                                  	call   1d9ad0 <<emuella_j2k_tier1::mq::Encoder>::byte_out>
  1d9cb5:	8b 4b 10                                        	mov    0x10(%rbx),%ecx
  1d9cb8:	66 85 c9                                        	test   %cx,%cx
  1d9cbb:	79 e3                                           	jns    1d9ca0 <<emuella_j2k_tier1::mq::Encoder>::write_bit+0xa0>
  1d9cbd:	5b                                              	pop    %rbx
  1d9cbe:	c3                                              	ret
  1d9cbf:	48 8d 15 1a 03 09 00                            	lea    0x9031a(%rip),%rdx        # 269fe0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3038>
  1d9cc6:	be 13 00 00 00                                  	mov    $0x13,%esi
  1d9ccb:	ff 15 d7 40 09 00                               	call   *0x940d7(%rip)        # 26dda8 <_DYNAMIC+0x228>
  1d9cd1:	48 8d 15 20 03 09 00                            	lea    0x90320(%rip),%rdx        # 269ff8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3050>
  1d9cd8:	be 2f 00 00 00                                  	mov    $0x2f,%esi
  1d9cdd:	48 89 c7                                        	mov    %rax,%rdi
  1d9ce0:	ff 15 c2 40 09 00                               	call   *0x940c2(%rip)        # 26dda8 <_DYNAMIC+0x228>
