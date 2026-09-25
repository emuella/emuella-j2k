Disassembly of section .text:

00000000001d9ad0 <<emuella_j2k_tier1::mq::Encoder>::byte_out>:
  1d9ad0:	55                                              	push   %rbp
  1d9ad1:	41 57                                           	push   %r15
  1d9ad3:	41 56                                           	push   %r14
  1d9ad5:	41 54                                           	push   %r12
  1d9ad7:	53                                              	push   %rbx
  1d9ad8:	44 0f b6 77 46                                  	movzbl 0x46(%rdi),%r14d
  1d9add:	80 7f 45 00                                     	cmpb   $0x0,0x45(%rdi)
  1d9ae1:	c6 47 45 00                                     	movb   $0x0,0x45(%rdi)
  1d9ae5:	74 4b                                           	je     1d9b32 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x62>
  1d9ae7:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1d9aeb:	74 67                                           	je     1d9b54 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x84>
  1d9aed:	8b 6f 14                                        	mov    0x14(%rdi),%ebp
  1d9af0:	81 fd ff ff ff 07                               	cmp    $0x7ffffff,%ebp
  1d9af6:	76 0d                                           	jbe    1d9b05 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x35>
  1d9af8:	41 fe c6                                        	inc    %r14b
  1d9afb:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1d9aff:	0f 84 94 00 00 00                               	je     1d9b99 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0xc9>
  1d9b05:	48 8b 5f 08                                     	mov    0x8(%rdi),%rbx
  1d9b09:	4c 8b 7b 10                                     	mov    0x10(%rbx),%r15
  1d9b0d:	4c 3b 3b                                        	cmp    (%rbx),%r15
  1d9b10:	75 0f                                           	jne    1d9b21 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x51>
  1d9b12:	49 89 fc                                        	mov    %rdi,%r12
  1d9b15:	48 89 df                                        	mov    %rbx,%rdi
  1d9b18:	ff 15 d2 42 09 00                               	call   *0x942d2(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1d9b1e:	4c 89 e7                                        	mov    %r12,%rdi
  1d9b21:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1d9b25:	46 88 34 38                                     	mov    %r14b,(%rax,%r15,1)
  1d9b29:	49 ff c7                                        	inc    %r15
  1d9b2c:	4c 89 7b 10                                     	mov    %r15,0x10(%rbx)
  1d9b30:	eb 03                                           	jmp    1d9b35 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x65>
  1d9b32:	8b 6f 14                                        	mov    0x14(%rdi),%ebp
  1d9b35:	89 e8                                           	mov    %ebp,%eax
  1d9b37:	c1 e8 13                                        	shr    $0x13,%eax
  1d9b3a:	c6 47 45 01                                     	movb   $0x1,0x45(%rdi)
  1d9b3e:	88 47 46                                        	mov    %al,0x46(%rdi)
  1d9b41:	81 e5 ff ff 07 00                               	and    $0x7ffff,%ebp
  1d9b47:	89 6f 14                                        	mov    %ebp,0x14(%rdi)
  1d9b4a:	b8 08 00 00 00                                  	mov    $0x8,%eax
  1d9b4f:	e9 99 00 00 00                                  	jmp    1d9bed <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x11d>
  1d9b54:	48 8b 5f 08                                     	mov    0x8(%rdi),%rbx
  1d9b58:	4c 8b 73 10                                     	mov    0x10(%rbx),%r14
  1d9b5c:	4c 3b 33                                        	cmp    (%rbx),%r14
  1d9b5f:	75 0f                                           	jne    1d9b70 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0xa0>
  1d9b61:	49 89 ff                                        	mov    %rdi,%r15
  1d9b64:	48 89 df                                        	mov    %rbx,%rdi
  1d9b67:	ff 15 83 42 09 00                               	call   *0x94283(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1d9b6d:	4c 89 ff                                        	mov    %r15,%rdi
  1d9b70:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1d9b74:	42 c6 04 30 ff                                  	movb   $0xff,(%rax,%r14,1)
  1d9b79:	49 ff c6                                        	inc    %r14
  1d9b7c:	4c 89 73 10                                     	mov    %r14,0x10(%rbx)
  1d9b80:	8b 47 14                                        	mov    0x14(%rdi),%eax
  1d9b83:	89 c1                                           	mov    %eax,%ecx
  1d9b85:	c1 e9 14                                        	shr    $0x14,%ecx
  1d9b88:	c6 47 45 01                                     	movb   $0x1,0x45(%rdi)
  1d9b8c:	88 4f 46                                        	mov    %cl,0x46(%rdi)
  1d9b8f:	25 ff ff 0f 00                                  	and    $0xfffff,%eax
  1d9b94:	89 47 14                                        	mov    %eax,0x14(%rdi)
  1d9b97:	eb 4f                                           	jmp    1d9be8 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x118>
  1d9b99:	41 89 ee                                        	mov    %ebp,%r14d
  1d9b9c:	41 81 e6 ff ff ff 07                            	and    $0x7ffffff,%r14d
  1d9ba3:	44 89 77 14                                     	mov    %r14d,0x14(%rdi)
  1d9ba7:	48 8b 5f 08                                     	mov    0x8(%rdi),%rbx
  1d9bab:	4c 8b 7b 10                                     	mov    0x10(%rbx),%r15
  1d9baf:	4c 3b 3b                                        	cmp    (%rbx),%r15
  1d9bb2:	75 0f                                           	jne    1d9bc3 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0xf3>
  1d9bb4:	49 89 fc                                        	mov    %rdi,%r12
  1d9bb7:	48 89 df                                        	mov    %rbx,%rdi
  1d9bba:	ff 15 30 42 09 00                               	call   *0x94230(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1d9bc0:	4c 89 e7                                        	mov    %r12,%rdi
  1d9bc3:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1d9bc7:	42 c6 04 38 ff                                  	movb   $0xff,(%rax,%r15,1)
  1d9bcc:	49 ff c7                                        	inc    %r15
  1d9bcf:	4c 89 7b 10                                     	mov    %r15,0x10(%rbx)
  1d9bd3:	41 c1 ee 14                                     	shr    $0x14,%r14d
  1d9bd7:	c6 47 45 01                                     	movb   $0x1,0x45(%rdi)
  1d9bdb:	44 88 77 46                                     	mov    %r14b,0x46(%rdi)
  1d9bdf:	81 e5 ff ff 0f 00                               	and    $0xfffff,%ebp
  1d9be5:	89 6f 14                                        	mov    %ebp,0x14(%rdi)
  1d9be8:	b8 07 00 00 00                                  	mov    $0x7,%eax
  1d9bed:	89 47 18                                        	mov    %eax,0x18(%rdi)
  1d9bf0:	5b                                              	pop    %rbx
  1d9bf1:	41 5c                                           	pop    %r12
  1d9bf3:	41 5e                                           	pop    %r14
  1d9bf5:	41 5f                                           	pop    %r15
  1d9bf7:	5d                                              	pop    %rbp
  1d9bf8:	c3                                              	ret
