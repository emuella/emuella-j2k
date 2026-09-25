Disassembly of section .text:

00000000001db4d0 <<emuella_j2k_tier1::mq::Encoder>::byte_out>:
  1db4d0:	55                                              	push   %rbp
  1db4d1:	41 57                                           	push   %r15
  1db4d3:	41 56                                           	push   %r14
  1db4d5:	41 54                                           	push   %r12
  1db4d7:	53                                              	push   %rbx
  1db4d8:	44 0f b6 77 46                                  	movzbl 0x46(%rdi),%r14d
  1db4dd:	80 7f 45 00                                     	cmpb   $0x0,0x45(%rdi)
  1db4e1:	c6 47 45 00                                     	movb   $0x0,0x45(%rdi)
  1db4e5:	74 4b                                           	je     1db532 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x62>
  1db4e7:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1db4eb:	74 67                                           	je     1db554 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x84>
  1db4ed:	8b 6f 14                                        	mov    0x14(%rdi),%ebp
  1db4f0:	81 fd ff ff ff 07                               	cmp    $0x7ffffff,%ebp
  1db4f6:	76 0d                                           	jbe    1db505 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x35>
  1db4f8:	41 fe c6                                        	inc    %r14b
  1db4fb:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1db4ff:	0f 84 94 00 00 00                               	je     1db599 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0xc9>
  1db505:	48 8b 5f 08                                     	mov    0x8(%rdi),%rbx
  1db509:	4c 8b 7b 10                                     	mov    0x10(%rbx),%r15
  1db50d:	4c 3b 3b                                        	cmp    (%rbx),%r15
  1db510:	75 0f                                           	jne    1db521 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x51>
  1db512:	49 89 fc                                        	mov    %rdi,%r12
  1db515:	48 89 df                                        	mov    %rbx,%rdi
  1db518:	ff 15 22 99 09 00                               	call   *0x99922(%rip)        # 274e40 <_DYNAMIC+0x290>
  1db51e:	4c 89 e7                                        	mov    %r12,%rdi
  1db521:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1db525:	46 88 34 38                                     	mov    %r14b,(%rax,%r15,1)
  1db529:	49 ff c7                                        	inc    %r15
  1db52c:	4c 89 7b 10                                     	mov    %r15,0x10(%rbx)
  1db530:	eb 03                                           	jmp    1db535 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x65>
  1db532:	8b 6f 14                                        	mov    0x14(%rdi),%ebp
  1db535:	89 e8                                           	mov    %ebp,%eax
  1db537:	c1 e8 13                                        	shr    $0x13,%eax
  1db53a:	c6 47 45 01                                     	movb   $0x1,0x45(%rdi)
  1db53e:	88 47 46                                        	mov    %al,0x46(%rdi)
  1db541:	81 e5 ff ff 07 00                               	and    $0x7ffff,%ebp
  1db547:	89 6f 14                                        	mov    %ebp,0x14(%rdi)
  1db54a:	b8 08 00 00 00                                  	mov    $0x8,%eax
  1db54f:	e9 99 00 00 00                                  	jmp    1db5ed <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x11d>
  1db554:	48 8b 5f 08                                     	mov    0x8(%rdi),%rbx
  1db558:	4c 8b 73 10                                     	mov    0x10(%rbx),%r14
  1db55c:	4c 3b 33                                        	cmp    (%rbx),%r14
  1db55f:	75 0f                                           	jne    1db570 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0xa0>
  1db561:	49 89 ff                                        	mov    %rdi,%r15
  1db564:	48 89 df                                        	mov    %rbx,%rdi
  1db567:	ff 15 d3 98 09 00                               	call   *0x998d3(%rip)        # 274e40 <_DYNAMIC+0x290>
  1db56d:	4c 89 ff                                        	mov    %r15,%rdi
  1db570:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1db574:	42 c6 04 30 ff                                  	movb   $0xff,(%rax,%r14,1)
  1db579:	49 ff c6                                        	inc    %r14
  1db57c:	4c 89 73 10                                     	mov    %r14,0x10(%rbx)
  1db580:	8b 47 14                                        	mov    0x14(%rdi),%eax
  1db583:	89 c1                                           	mov    %eax,%ecx
  1db585:	c1 e9 14                                        	shr    $0x14,%ecx
  1db588:	c6 47 45 01                                     	movb   $0x1,0x45(%rdi)
  1db58c:	88 4f 46                                        	mov    %cl,0x46(%rdi)
  1db58f:	25 ff ff 0f 00                                  	and    $0xfffff,%eax
  1db594:	89 47 14                                        	mov    %eax,0x14(%rdi)
  1db597:	eb 4f                                           	jmp    1db5e8 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0x118>
  1db599:	41 89 ee                                        	mov    %ebp,%r14d
  1db59c:	41 81 e6 ff ff ff 07                            	and    $0x7ffffff,%r14d
  1db5a3:	44 89 77 14                                     	mov    %r14d,0x14(%rdi)
  1db5a7:	48 8b 5f 08                                     	mov    0x8(%rdi),%rbx
  1db5ab:	4c 8b 7b 10                                     	mov    0x10(%rbx),%r15
  1db5af:	4c 3b 3b                                        	cmp    (%rbx),%r15
  1db5b2:	75 0f                                           	jne    1db5c3 <<emuella_j2k_tier1::mq::Encoder>::byte_out+0xf3>
  1db5b4:	49 89 fc                                        	mov    %rdi,%r12
  1db5b7:	48 89 df                                        	mov    %rbx,%rdi
  1db5ba:	ff 15 80 98 09 00                               	call   *0x99880(%rip)        # 274e40 <_DYNAMIC+0x290>
  1db5c0:	4c 89 e7                                        	mov    %r12,%rdi
  1db5c3:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1db5c7:	42 c6 04 38 ff                                  	movb   $0xff,(%rax,%r15,1)
  1db5cc:	49 ff c7                                        	inc    %r15
  1db5cf:	4c 89 7b 10                                     	mov    %r15,0x10(%rbx)
  1db5d3:	41 c1 ee 14                                     	shr    $0x14,%r14d
  1db5d7:	c6 47 45 01                                     	movb   $0x1,0x45(%rdi)
  1db5db:	44 88 77 46                                     	mov    %r14b,0x46(%rdi)
  1db5df:	81 e5 ff ff 0f 00                               	and    $0xfffff,%ebp
  1db5e5:	89 6f 14                                        	mov    %ebp,0x14(%rdi)
  1db5e8:	b8 07 00 00 00                                  	mov    $0x7,%eax
  1db5ed:	89 47 18                                        	mov    %eax,0x18(%rdi)
  1db5f0:	5b                                              	pop    %rbx
  1db5f1:	41 5c                                           	pop    %r12
  1db5f3:	41 5e                                           	pop    %r14
  1db5f5:	41 5f                                           	pop    %r15
  1db5f7:	5d                                              	pop    %rbp
  1db5f8:	c3                                              	ret
