Disassembly of section .text:

000000000006d400 <classic_compare_worker::encode>:
   6d400:	55                                              	push   %rbp
   6d401:	41 57                                           	push   %r15
   6d403:	41 56                                           	push   %r14
   6d405:	41 55                                           	push   %r13
   6d407:	41 54                                           	push   %r12
   6d409:	53                                              	push   %rbx
   6d40a:	48 81 ec c8 00 00 00                            	sub    $0xc8,%rsp
   6d411:	49 89 cd                                        	mov    %rcx,%r13
   6d414:	49 89 d4                                        	mov    %rdx,%r12
   6d417:	49 89 f7                                        	mov    %rsi,%r15
   6d41a:	48 83 7e 10 07                                  	cmpq   $0x7,0x10(%rsi)
   6d41f:	75 1b                                           	jne    6d43c <classic_compare_worker::encode+0x3c>
   6d421:	49 8b 47 08                                     	mov    0x8(%r15),%rax
   6d425:	b9 65 6d 75 65                                  	mov    $0x65756d65,%ecx
   6d42a:	33 08                                           	xor    (%rax),%ecx
   6d42c:	ba 65 6c 6c 61                                  	mov    $0x616c6c65,%edx
   6d431:	33 50 03                                        	xor    0x3(%rax),%edx
   6d434:	09 ca                                           	or     %ecx,%edx
   6d436:	0f 84 d0 00 00 00                               	je     6d50c <classic_compare_worker::encode+0x10c>
   6d43c:	41 0f b6 af ca 00 00 00                         	movzbl 0xca(%r15),%ebp
   6d444:	83 fd 08                                        	cmp    $0x8,%ebp
   6d447:	75 36                                           	jne    6d47f <classic_compare_worker::encode+0x7f>
   6d449:	48 89 7c 24 38                                  	mov    %rdi,0x38(%rsp)
   6d44e:	4a 8d 1c ad 00 00 00 00                         	lea    0x0(,%r13,4),%rbx
   6d456:	4c 89 e8                                        	mov    %r13,%rax
   6d459:	48 c1 e8 3e                                     	shr    $0x3e,%rax
   6d45d:	0f 95 c0                                        	setne  %al
   6d460:	48 b9 fc ff ff ff ff ff ff 7f                   	movabs $0x7ffffffffffffffc,%rcx
   6d46a:	48 39 cb                                        	cmp    %rcx,%rbx
   6d46d:	0f 97 c1                                        	seta   %cl
   6d470:	08 c1                                           	or     %al,%cl
   6d472:	74 2a                                           	je     6d49e <classic_compare_worker::encode+0x9e>
   6d474:	31 ff                                           	xor    %edi,%edi
   6d476:	48 89 de                                        	mov    %rbx,%rsi
   6d479:	ff 15 19 09 20 00                               	call   *0x200919(%rip)        # 26dd98 <_DYNAMIC+0x218>
   6d47f:	4c 89 e9                                        	mov    %r13,%rcx
   6d482:	48 d1 e9                                        	shr    $1,%rcx
   6d485:	48 8d 34 8d 00 00 00 00                         	lea    0x0(,%rcx,4),%rsi
   6d48d:	4c 89 e8                                        	mov    %r13,%rax
   6d490:	48 c1 e8 3e                                     	shr    $0x3e,%rax
   6d494:	74 3c                                           	je     6d4d2 <classic_compare_worker::encode+0xd2>
   6d496:	31 ff                                           	xor    %edi,%edi
   6d498:	ff 15 fa 08 20 00                               	call   *0x2008fa(%rip)        # 26dd98 <_DYNAMIC+0x218>
   6d49e:	48 85 db                                        	test   %rbx,%rbx
   6d4a1:	0f 84 a7 00 00 00                               	je     6d54e <classic_compare_worker::encode+0x14e>
   6d4a7:	48 89 df                                        	mov    %rbx,%rdi
   6d4aa:	ff 15 38 09 20 00                               	call   *0x200938(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   6d4b0:	49 89 c6                                        	mov    %rax,%r14
   6d4b3:	4c 89 ea                                        	mov    %r13,%rdx
   6d4b6:	48 85 c0                                        	test   %rax,%rax
   6d4b9:	0f 84 cb 06 00 00                               	je     6db8a <classic_compare_worker::encode+0x78a>
   6d4bf:	4d 85 ed                                        	test   %r13,%r13
   6d4c2:	48 89 54 24 30                                  	mov    %rdx,0x30(%rsp)
   6d4c7:	0f 85 97 00 00 00                               	jne    6d564 <classic_compare_worker::encode+0x164>
   6d4cd:	e9 b2 04 00 00                                  	jmp    6d984 <classic_compare_worker::encode+0x584>
   6d4d2:	48 89 7c 24 38                                  	mov    %rdi,0x38(%rsp)
   6d4d7:	48 89 4c 24 30                                  	mov    %rcx,0x30(%rsp)
   6d4dc:	48 85 c9                                        	test   %rcx,%rcx
   6d4df:	0f 84 13 01 00 00                               	je     6d5f8 <classic_compare_worker::encode+0x1f8>
   6d4e5:	48 89 f3                                        	mov    %rsi,%rbx
   6d4e8:	48 89 f7                                        	mov    %rsi,%rdi
   6d4eb:	ff 15 f7 08 20 00                               	call   *0x2008f7(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   6d4f1:	49 89 c6                                        	mov    %rax,%r14
   6d4f4:	48 85 c0                                        	test   %rax,%rax
   6d4f7:	0f 84 8d 06 00 00                               	je     6db8a <classic_compare_worker::encode+0x78a>
   6d4fd:	49 83 fd 02                                     	cmp    $0x2,%r13
   6d501:	0f 83 01 01 00 00                               	jae    6d608 <classic_compare_worker::encode+0x208>
   6d507:	e9 78 04 00 00                                  	jmp    6d984 <classic_compare_worker::encode+0x584>
   6d50c:	48 89 fb                                        	mov    %rdi,%rbx
   6d50f:	49 89 e6                                        	mov    %rsp,%r14
   6d512:	4c 89 f7                                        	mov    %r14,%rdi
   6d515:	4c 89 fe                                        	mov    %r15,%rsi
   6d518:	e8 d3 16 00 00                                  	call   6ebf0 <<classic_compare_worker::Request>::info>
   6d51d:	48 8b 04 24                                     	mov    (%rsp),%rax
   6d521:	0f 10 44 24 08                                  	movups 0x8(%rsp),%xmm0
   6d526:	0f 29 44 24 40                                  	movaps %xmm0,0x40(%rsp)
   6d52b:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
   6d52f:	0f 84 ab 01 00 00                               	je     6d6e0 <classic_compare_worker::encode+0x2e0>
   6d535:	0f 28 44 24 40                                  	movaps 0x40(%rsp),%xmm0
   6d53a:	0f 11 43 10                                     	movups %xmm0,0x10(%rbx)
   6d53e:	48 89 43 08                                     	mov    %rax,0x8(%rbx)
   6d542:	48 c7 03 01 00 00 00                            	movq   $0x1,(%rbx)
   6d549:	e9 61 05 00 00                                  	jmp    6daaf <classic_compare_worker::encode+0x6af>
   6d54e:	41 be 04 00 00 00                               	mov    $0x4,%r14d
   6d554:	31 d2                                           	xor    %edx,%edx
   6d556:	4d 85 ed                                        	test   %r13,%r13
   6d559:	48 89 54 24 30                                  	mov    %rdx,0x30(%rsp)
   6d55e:	0f 84 20 04 00 00                               	je     6d984 <classic_compare_worker::encode+0x584>
   6d564:	49 83 fd 08                                     	cmp    $0x8,%r13
   6d568:	72 1b                                           	jb     6d585 <classic_compare_worker::encode+0x185>
   6d56a:	4c 01 f3                                        	add    %r14,%rbx
   6d56d:	4b 8d 04 2c                                     	lea    (%r12,%r13,1),%rax
   6d571:	49 39 c6                                        	cmp    %rax,%r14
   6d574:	0f 92 c0                                        	setb   %al
   6d577:	49 39 dc                                        	cmp    %rbx,%r12
   6d57a:	0f 92 c1                                        	setb   %cl
   6d57d:	84 c8                                           	test   %cl,%al
   6d57f:	0f 84 ae 01 00 00                               	je     6d733 <classic_compare_worker::encode+0x333>
   6d585:	31 c0                                           	xor    %eax,%eax
   6d587:	4c 89 ea                                        	mov    %r13,%rdx
   6d58a:	48 89 c1                                        	mov    %rax,%rcx
   6d58d:	48 83 e2 03                                     	and    $0x3,%rdx
   6d591:	74 1e                                           	je     6d5b1 <classic_compare_worker::encode+0x1b1>
   6d593:	48 89 c1                                        	mov    %rax,%rcx
   6d596:	66 2e 0f 1f 84 00 00 00 00 00                   	cs nopw 0x0(%rax,%rax,1)
   6d5a0:	41 0f b6 34 0c                                  	movzbl (%r12,%rcx,1),%esi
   6d5a5:	41 89 34 8e                                     	mov    %esi,(%r14,%rcx,4)
   6d5a9:	48 ff c1                                        	inc    %rcx
   6d5ac:	48 ff ca                                        	dec    %rdx
   6d5af:	75 ef                                           	jne    6d5a0 <classic_compare_worker::encode+0x1a0>
   6d5b1:	4c 29 e8                                        	sub    %r13,%rax
   6d5b4:	48 83 f8 fc                                     	cmp    $0xfffffffffffffffc,%rax
   6d5b8:	0f 87 c6 03 00 00                               	ja     6d984 <classic_compare_worker::encode+0x584>
   6d5be:	66 90                                           	xchg   %ax,%ax
   6d5c0:	41 0f b6 04 0c                                  	movzbl (%r12,%rcx,1),%eax
   6d5c5:	41 89 04 8e                                     	mov    %eax,(%r14,%rcx,4)
   6d5c9:	41 0f b6 44 0c 01                               	movzbl 0x1(%r12,%rcx,1),%eax
   6d5cf:	41 89 44 8e 04                                  	mov    %eax,0x4(%r14,%rcx,4)
   6d5d4:	41 0f b6 44 0c 02                               	movzbl 0x2(%r12,%rcx,1),%eax
   6d5da:	41 89 44 8e 08                                  	mov    %eax,0x8(%r14,%rcx,4)
   6d5df:	41 0f b6 44 0c 03                               	movzbl 0x3(%r12,%rcx,1),%eax
   6d5e5:	41 89 44 8e 0c                                  	mov    %eax,0xc(%r14,%rcx,4)
   6d5ea:	48 83 c1 04                                     	add    $0x4,%rcx
   6d5ee:	49 39 cd                                        	cmp    %rcx,%r13
   6d5f1:	75 cd                                           	jne    6d5c0 <classic_compare_worker::encode+0x1c0>
   6d5f3:	e9 8c 03 00 00                                  	jmp    6d984 <classic_compare_worker::encode+0x584>
   6d5f8:	41 be 04 00 00 00                               	mov    $0x4,%r14d
   6d5fe:	49 83 fd 02                                     	cmp    $0x2,%r13
   6d602:	0f 82 7c 03 00 00                               	jb     6d984 <classic_compare_worker::encode+0x584>
   6d608:	48 b9 fc ff ff ff ff ff ff 7f                   	movabs $0x7ffffffffffffffc,%rcx
   6d612:	48 8d 41 02                                     	lea    0x2(%rcx),%rax
   6d616:	4c 21 e8                                        	and    %r13,%rax
   6d619:	49 8d 75 fe                                     	lea    -0x2(%r13),%rsi
   6d61d:	48 83 fe 1e                                     	cmp    $0x1e,%rsi
   6d621:	72 1f                                           	jb     6d642 <classic_compare_worker::encode+0x242>
   6d623:	49 8d 14 04                                     	lea    (%r12,%rax,1),%rdx
   6d627:	49 39 d6                                        	cmp    %rdx,%r14
   6d62a:	0f 83 ff 02 00 00                               	jae    6d92f <classic_compare_worker::encode+0x52f>
   6d630:	4d 01 ed                                        	add    %r13,%r13
   6d633:	49 21 cd                                        	and    %rcx,%r13
   6d636:	4d 01 f5                                        	add    %r14,%r13
   6d639:	4d 39 ec                                        	cmp    %r13,%r12
   6d63c:	0f 83 ed 02 00 00                               	jae    6d92f <classic_compare_worker::encode+0x52f>
   6d642:	31 d2                                           	xor    %edx,%edx
   6d644:	4c 89 e1                                        	mov    %r12,%rcx
   6d647:	48 8d 70 fe                                     	lea    -0x2(%rax),%rsi
   6d64b:	89 f7                                           	mov    %esi,%edi
   6d64d:	f7 d7                                           	not    %edi
   6d64f:	40 f6 c7 06                                     	test   $0x6,%dil
   6d653:	74 39                                           	je     6d68e <classic_compare_worker::encode+0x28e>
   6d655:	89 f7                                           	mov    %esi,%edi
   6d657:	d1 ef                                           	shr    $1,%edi
   6d659:	ff c7                                           	inc    %edi
   6d65b:	83 e7 03                                        	and    $0x3,%edi
   6d65e:	4d 8d 0c 96                                     	lea    (%r14,%rdx,4),%r9
   6d662:	48 f7 df                                        	neg    %rdi
   6d665:	45 31 c0                                        	xor    %r8d,%r8d
   6d668:	45 31 d2                                        	xor    %r10d,%r10d
   6d66b:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
   6d670:	46 0f b7 1c 01                                  	movzwl (%rcx,%r8,1),%r11d
   6d675:	47 89 1c 41                                     	mov    %r11d,(%r9,%r8,2)
   6d679:	49 ff ca                                        	dec    %r10
   6d67c:	49 83 c0 02                                     	add    $0x2,%r8
   6d680:	4c 39 d7                                        	cmp    %r10,%rdi
   6d683:	75 eb                                           	jne    6d670 <classic_compare_worker::encode+0x270>
   6d685:	4c 29 d2                                        	sub    %r10,%rdx
   6d688:	4c 29 c0                                        	sub    %r8,%rax
   6d68b:	4c 01 c1                                        	add    %r8,%rcx
   6d68e:	48 83 fe 06                                     	cmp    $0x6,%rsi
   6d692:	0f 82 ec 02 00 00                               	jb     6d984 <classic_compare_worker::encode+0x584>
   6d698:	49 8d 14 96                                     	lea    (%r14,%rdx,4),%rdx
   6d69c:	48 83 c2 0c                                     	add    $0xc,%rdx
   6d6a0:	31 f6                                           	xor    %esi,%esi
   6d6a2:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   6d6b0:	0f b7 3c 31                                     	movzwl (%rcx,%rsi,1),%edi
   6d6b4:	89 7c 72 f4                                     	mov    %edi,-0xc(%rdx,%rsi,2)
   6d6b8:	0f b7 7c 31 02                                  	movzwl 0x2(%rcx,%rsi,1),%edi
   6d6bd:	89 7c 72 f8                                     	mov    %edi,-0x8(%rdx,%rsi,2)
   6d6c1:	0f b7 7c 31 04                                  	movzwl 0x4(%rcx,%rsi,1),%edi
   6d6c6:	89 7c 72 fc                                     	mov    %edi,-0x4(%rdx,%rsi,2)
   6d6ca:	0f b7 7c 31 06                                  	movzwl 0x6(%rcx,%rsi,1),%edi
   6d6cf:	89 3c 72                                        	mov    %edi,(%rdx,%rsi,2)
   6d6d2:	48 83 c6 08                                     	add    $0x8,%rsi
   6d6d6:	48 39 f0                                        	cmp    %rsi,%rax
   6d6d9:	75 d5                                           	jne    6d6b0 <classic_compare_worker::encode+0x2b0>
   6d6db:	e9 a4 02 00 00                                  	jmp    6d984 <classic_compare_worker::encode+0x584>
   6d6e0:	0f 28 44 24 40                                  	movaps 0x40(%rsp),%xmm0
   6d6e5:	0f 29 84 24 a0 00 00 00                         	movaps %xmm0,0xa0(%rsp)
   6d6ed:	41 8b 8f c4 00 00 00                            	mov    0xc4(%r15),%ecx
   6d6f4:	48 85 c9                                        	test   %rcx,%rcx
   6d6f7:	0f 84 9b 04 00 00                               	je     6db98 <classic_compare_worker::encode+0x798>
   6d6fd:	41 8b 97 c0 00 00 00                            	mov    0xc0(%r15),%edx
   6d704:	48 0f af d1                                     	imul   %rcx,%rdx
   6d708:	41 0f b7 b7 c8 00 00 00                         	movzwl 0xc8(%r15),%esi
   6d710:	41 0f b6 87 ca 00 00 00                         	movzbl 0xca(%r15),%eax
   6d718:	c1 e8 03                                        	shr    $0x3,%eax
   6d71b:	48 0f af c6                                     	imul   %rsi,%rax
   6d71f:	48 0f af c2                                     	imul   %rdx,%rax
   6d723:	48 89 c2                                        	mov    %rax,%rdx
   6d726:	48 c1 ea 20                                     	shr    $0x20,%rdx
   6d72a:	74 65                                           	je     6d791 <classic_compare_worker::encode+0x391>
   6d72c:	31 d2                                           	xor    %edx,%edx
   6d72e:	48 f7 f1                                        	div    %rcx
   6d731:	eb 62                                           	jmp    6d795 <classic_compare_worker::encode+0x395>
   6d733:	48 b8 ff ff ff ff ff ff ff 3f                   	movabs $0x3fffffffffffffff,%rax
   6d73d:	48 83 c0 f9                                     	add    $0xfffffffffffffff9,%rax
   6d741:	4c 21 e8                                        	and    %r13,%rax
   6d744:	31 c9                                           	xor    %ecx,%ecx
   6d746:	66 0f ef c0                                     	pxor   %xmm0,%xmm0
   6d74a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
   6d750:	66 41 0f 6e 0c 0c                               	movd   (%r12,%rcx,1),%xmm1
   6d756:	66 41 0f 6e 54 0c 04                            	movd   0x4(%r12,%rcx,1),%xmm2
   6d75d:	66 0f 60 c8                                     	punpcklbw %xmm0,%xmm1
   6d761:	66 0f 61 c8                                     	punpcklwd %xmm0,%xmm1
   6d765:	66 0f 60 d0                                     	punpcklbw %xmm0,%xmm2
   6d769:	66 0f 61 d0                                     	punpcklwd %xmm0,%xmm2
   6d76d:	f3 41 0f 7f 0c 8e                               	movdqu %xmm1,(%r14,%rcx,4)
   6d773:	f3 41 0f 7f 54 8e 10                            	movdqu %xmm2,0x10(%r14,%rcx,4)
   6d77a:	48 83 c1 08                                     	add    $0x8,%rcx
   6d77e:	48 39 c8                                        	cmp    %rcx,%rax
   6d781:	75 cd                                           	jne    6d750 <classic_compare_worker::encode+0x350>
   6d783:	49 39 c5                                        	cmp    %rax,%r13
   6d786:	0f 84 f8 01 00 00                               	je     6d984 <classic_compare_worker::encode+0x584>
   6d78c:	e9 f6 fd ff ff                                  	jmp    6d587 <classic_compare_worker::encode+0x187>
   6d791:	31 d2                                           	xor    %edx,%edx
   6d793:	f7 f1                                           	div    %ecx
   6d795:	48 8d 8c 24 a0 00 00 00                         	lea    0xa0(%rsp),%rcx
   6d79d:	48 89 4c 24 70                                  	mov    %rcx,0x70(%rsp)
   6d7a2:	4c 89 64 24 78                                  	mov    %r12,0x78(%rsp)
   6d7a7:	4c 89 ac 24 80 00 00 00                         	mov    %r13,0x80(%rsp)
   6d7af:	48 89 84 24 88 00 00 00                         	mov    %rax,0x88(%rsp)
   6d7b7:	49 8d 8f b0 00 00 00                            	lea    0xb0(%r15),%rcx
   6d7be:	41 80 bf cb 00 00 00 00                         	cmpb   $0x0,0xcb(%r15)
   6d7c6:	74 47                                           	je     6d80f <classic_compare_worker::encode+0x40f>
   6d7c8:	c7 44 24 18 00 00 00 00                         	movl   $0x0,0x18(%rsp)
   6d7d0:	c7 44 24 2c 01 00 00 02                         	movl   $0x2000001,0x2c(%rsp)
   6d7d8:	c7 44 24 20 00 00 00 00                         	movl   $0x0,0x20(%rsp)
   6d7e0:	48 c7 04 24 00 00 00 00                         	movq   $0x0,(%rsp)
   6d7e8:	48 c7 44 24 08 08 00 00 00                      	movq   $0x8,0x8(%rsp)
   6d7f1:	48 c7 44 24 10 00 00 00 00                      	movq   $0x0,0x10(%rsp)
   6d7fa:	48 8d 7c 24 40                                  	lea    0x40(%rsp),%rdi
   6d7ff:	48 8d 74 24 70                                  	lea    0x70(%rsp),%rsi
   6d804:	48 89 e2                                        	mov    %rsp,%rdx
   6d807:	ff 15 8b 07 20 00                               	call   *0x20078b(%rip)        # 26df98 <_DYNAMIC+0x418>
   6d80d:	eb 45                                           	jmp    6d854 <classic_compare_worker::encode+0x454>
   6d80f:	c7 44 24 18 00 00 00 00                         	movl   $0x0,0x18(%rsp)
   6d817:	c7 44 24 2c 01 00 00 02                         	movl   $0x2000001,0x2c(%rsp)
   6d81f:	c7 44 24 20 00 00 00 00                         	movl   $0x0,0x20(%rsp)
   6d827:	48 c7 04 24 00 00 00 00                         	movq   $0x0,(%rsp)
   6d82f:	48 c7 44 24 08 08 00 00 00                      	movq   $0x8,0x8(%rsp)
   6d838:	48 c7 44 24 10 00 00 00 00                      	movq   $0x0,0x10(%rsp)
   6d841:	48 8d 7c 24 40                                  	lea    0x40(%rsp),%rdi
   6d846:	48 8d 74 24 70                                  	lea    0x70(%rsp),%rsi
   6d84b:	48 89 e2                                        	mov    %rsp,%rdx
   6d84e:	ff 15 4c 07 20 00                               	call   *0x20074c(%rip)        # 26dfa0 <_DYNAMIC+0x420>
   6d854:	48 89 d9                                        	mov    %rbx,%rcx
   6d857:	48 83 7c 24 40 ff                               	cmpq   $0xffffffffffffffff,0x40(%rsp)
   6d85d:	0f 84 ae 00 00 00                               	je     6d911 <classic_compare_worker::encode+0x511>
   6d863:	0f 10 44 24 40                                  	movups 0x40(%rsp),%xmm0
   6d868:	f3 0f 6f 4c 24 50                               	movdqu 0x50(%rsp),%xmm1
   6d86e:	f3 0f 6f 54 24 60                               	movdqu 0x60(%rsp),%xmm2
   6d874:	66 0f 7f 54 24 20                               	movdqa %xmm2,0x20(%rsp)
   6d87a:	66 0f 7f 4c 24 10                               	movdqa %xmm1,0x10(%rsp)
   6d880:	0f 29 04 24                                     	movaps %xmm0,(%rsp)
   6d884:	4c 89 b4 24 90 00 00 00                         	mov    %r14,0x90(%rsp)
   6d88c:	48 8d 05 6d 39 00 00                            	lea    0x396d(%rip),%rax        # 71200 <<emuella_j2k_core::J2kError as core::fmt::Debug>::fmt>
   6d893:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
   6d89b:	48 8d 35 19 08 fa ff                            	lea    -0x5f7e7(%rip),%rsi        # e0bb <anon.b5c622d8c0861678d1a06bfbb6410fee.144.llvm.5232537515227879042>
   6d8a2:	48 8d bc 24 b0 00 00 00                         	lea    0xb0(%rsp),%rdi
   6d8aa:	48 8d 94 24 90 00 00 00                         	lea    0x90(%rsp),%rdx
   6d8b2:	ff 15 78 05 20 00                               	call   *0x200578(%rip)        # 26de30 <_DYNAMIC+0x2b0>
   6d8b8:	48 8d 44 24 10                                  	lea    0x10(%rsp),%rax
   6d8bd:	48 8b 0c 24                                     	mov    (%rsp),%rcx
   6d8c1:	48 be fc ff ff ff ff ff ff 7f                   	movabs $0x7ffffffffffffffc,%rsi
   6d8cb:	48 83 c6 04                                     	add    $0x4,%rsi
   6d8cf:	48 31 ce                                        	xor    %rcx,%rsi
   6d8d2:	48 85 c9                                        	test   %rcx,%rcx
   6d8d5:	ba 02 00 00 00                                  	mov    $0x2,%edx
   6d8da:	48 0f 48 d6                                     	cmovs  %rsi,%rdx
   6d8de:	48 83 fa 05                                     	cmp    $0x5,%rdx
   6d8e2:	0f 87 51 02 00 00                               	ja     6db39 <classic_compare_worker::encode+0x739>
   6d8e8:	48 8d 35 71 33 fa ff                            	lea    -0x5cc8f(%rip),%rsi        # 10c60 <anon.ee651107ab5319c6bc273e1a29320aaf.108.llvm.14746981713632465754+0x40>
   6d8ef:	48 63 14 96                                     	movslq (%rsi,%rdx,4),%rdx
   6d8f3:	48 01 f2                                        	add    %rsi,%rdx
   6d8f6:	48 89 de                                        	mov    %rbx,%rsi
   6d8f9:	ff e2                                           	jmp    *%rdx
   6d8fb:	48 83 7c 24 18 00                               	cmpq   $0x0,0x18(%rsp)
   6d901:	0f 84 55 02 00 00                               	je     6db5c <classic_compare_worker::encode+0x75c>
   6d907:	48 8d 44 24 20                                  	lea    0x20(%rsp),%rax
   6d90c:	e9 3f 02 00 00                                  	jmp    6db50 <classic_compare_worker::encode+0x750>
   6d911:	48 8b 44 24 58                                  	mov    0x58(%rsp),%rax
   6d916:	48 89 41 18                                     	mov    %rax,0x18(%rcx)
   6d91a:	0f 10 44 24 48                                  	movups 0x48(%rsp),%xmm0
   6d91f:	0f 11 41 08                                     	movups %xmm0,0x8(%rcx)
   6d923:	48 c7 01 00 00 00 00                            	movq   $0x0,(%rcx)
   6d92a:	e9 80 01 00 00                                  	jmp    6daaf <classic_compare_worker::encode+0x6af>
   6d92f:	48 d1 ee                                        	shr    $1,%rsi
   6d932:	48 ff c6                                        	inc    %rsi
   6d935:	48 89 f2                                        	mov    %rsi,%rdx
   6d938:	48 83 e2 f8                                     	and    $0xfffffffffffffff8,%rdx
   6d93c:	48 29 d0                                        	sub    %rdx,%rax
   6d93f:	48 29 d0                                        	sub    %rdx,%rax
   6d942:	49 8d 0c 54                                     	lea    (%r12,%rdx,2),%rcx
   6d946:	31 ff                                           	xor    %edi,%edi
   6d948:	66 0f ef c0                                     	pxor   %xmm0,%xmm0
   6d94c:	0f 1f 40 00                                     	nopl   0x0(%rax)
   6d950:	f3 41 0f 7e 0c 7c                               	movq   (%r12,%rdi,2),%xmm1
   6d956:	f3 41 0f 7e 54 7c 08                            	movq   0x8(%r12,%rdi,2),%xmm2
   6d95d:	66 0f 61 c8                                     	punpcklwd %xmm0,%xmm1
   6d961:	66 0f 61 d0                                     	punpcklwd %xmm0,%xmm2
   6d965:	f3 41 0f 7f 0c be                               	movdqu %xmm1,(%r14,%rdi,4)
   6d96b:	f3 41 0f 7f 54 be 10                            	movdqu %xmm2,0x10(%r14,%rdi,4)
   6d972:	48 83 c7 08                                     	add    $0x8,%rdi
   6d976:	48 39 fa                                        	cmp    %rdi,%rdx
   6d979:	75 d5                                           	jne    6d950 <classic_compare_worker::encode+0x550>
   6d97b:	48 39 d6                                        	cmp    %rdx,%rsi
   6d97e:	0f 85 c3 fc ff ff                               	jne    6d647 <classic_compare_worker::encode+0x247>
   6d984:	48 c7 44 24 40 00 00 00 00                      	movq   $0x0,0x40(%rsp)
   6d98d:	48 c7 04 24 00 00 00 00                         	movq   $0x0,(%rsp)
   6d995:	41 8b b7 c0 00 00 00                            	mov    0xc0(%r15),%esi
   6d99c:	41 8b 97 c4 00 00 00                            	mov    0xc4(%r15),%edx
   6d9a3:	41 0f b7 8f c8 00 00 00                         	movzwl 0xc8(%r15),%ecx
   6d9ab:	31 c0                                           	xor    %eax,%eax
   6d9ad:	83 f9 03                                        	cmp    $0x3,%ecx
   6d9b0:	0f 94 c0                                        	sete   %al
   6d9b3:	45 0f b6 97 cc 00 00 00                         	movzbl 0xcc(%r15),%r10d
   6d9bb:	45 0f b6 9f cb 00 00 00                         	movzbl 0xcb(%r15),%r11d
   6d9c3:	48 89 e3                                        	mov    %rsp,%rbx
   6d9c6:	4c 8d 64 24 40                                  	lea    0x40(%rsp),%r12
   6d9cb:	f2 0f 10 05 05 32 fa ff                         	movsd  -0x5cdfb(%rip),%xmm0        # 10bd8 <anon.163a7daa153173e3324b1e1e59ef13e5.3.llvm.12432551406381563140+0x398>
   6d9d3:	4c 89 f7                                        	mov    %r14,%rdi
   6d9d6:	41 89 e8                                        	mov    %ebp,%r8d
   6d9d9:	41 b9 02 00 00 00                               	mov    $0x2,%r9d
   6d9df:	53                                              	push   %rbx
   6d9e0:	41 54                                           	push   %r12
   6d9e2:	50                                              	push   %rax
   6d9e3:	41 53                                           	push   %r11
   6d9e5:	41 52                                           	push   %r10
   6d9e7:	6a 01                                           	push   $0x1
   6d9e9:	ff 15 b9 05 20 00                               	call   *0x2005b9(%rip)        # 26dfa8 <_DYNAMIC+0x428>
   6d9ef:	48 83 c4 30                                     	add    $0x30,%rsp
   6d9f3:	85 c0                                           	test   %eax,%eax
   6d9f5:	74 58                                           	je     6da4f <classic_compare_worker::encode+0x64f>
   6d9f7:	4c 8b 24 24                                     	mov    (%rsp),%r12
   6d9fb:	48 8b 5c 24 40                                  	mov    0x40(%rsp),%rbx
   6da00:	4d 3b a7 b8 00 00 00                            	cmp    0xb8(%r15),%r12
   6da07:	0f 86 b4 00 00 00                               	jbe    6dac1 <classic_compare_worker::encode+0x6c1>
   6da0d:	48 89 df                                        	mov    %rbx,%rdi
   6da10:	ff 15 7a 05 20 00                               	call   *0x20057a(%rip)        # 26df90 <_DYNAMIC+0x410>
   6da16:	41 bc 15 00 00 00                               	mov    $0x15,%r12d
   6da1c:	bf 15 00 00 00                                  	mov    $0x15,%edi
   6da21:	ff 15 c1 03 20 00                               	call   *0x2003c1(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   6da27:	48 85 c0                                        	test   %rax,%rax
   6da2a:	0f 84 50 01 00 00                               	je     6db80 <classic_compare_worker::encode+0x780>
   6da30:	0f 10 05 28 35 fa ff                            	movups -0x5cad8(%rip),%xmm0        # 10f5f <anon.ee651107ab5319c6bc273e1a29320aaf.108.llvm.14746981713632465754+0x33f>
   6da37:	0f 11 00                                        	movups %xmm0,(%rax)
   6da3a:	48 b9 65 78 63 65 65 64 65 64                   	movabs $0x6465646565637865,%rcx
   6da44:	48 89 48 0d                                     	mov    %rcx,0xd(%rax)
   6da48:	b9 15 00 00 00                                  	mov    $0x15,%ecx
   6da4d:	eb 37                                           	jmp    6da86 <classic_compare_worker::encode+0x686>
   6da4f:	41 bc 16 00 00 00                               	mov    $0x16,%r12d
   6da55:	bf 16 00 00 00                                  	mov    $0x16,%edi
   6da5a:	ff 15 88 03 20 00                               	call   *0x200388(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   6da60:	48 85 c0                                        	test   %rax,%rax
   6da63:	0f 84 17 01 00 00                               	je     6db80 <classic_compare_worker::encode+0x780>
   6da69:	0f 10 05 d9 36 fa ff                            	movups -0x5c927(%rip),%xmm0        # 11149 <anon.ee651107ab5319c6bc273e1a29320aaf.108.llvm.14746981713632465754+0x529>
   6da70:	0f 11 00                                        	movups %xmm0,(%rax)
   6da73:	48 b9 65 20 66 61 69 6c 65 64                   	movabs $0x64656c6961662065,%rcx
   6da7d:	48 89 48 0e                                     	mov    %rcx,0xe(%rax)
   6da81:	b9 16 00 00 00                                  	mov    $0x16,%ecx
   6da86:	48 8b 54 24 38                                  	mov    0x38(%rsp),%rdx
   6da8b:	48 89 4a 08                                     	mov    %rcx,0x8(%rdx)
   6da8f:	48 89 42 10                                     	mov    %rax,0x10(%rdx)
   6da93:	48 89 4a 18                                     	mov    %rcx,0x18(%rdx)
   6da97:	48 c7 02 01 00 00 00                            	movq   $0x1,(%rdx)
   6da9e:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   6daa4:	74 09                                           	je     6daaf <classic_compare_worker::encode+0x6af>
   6daa6:	4c 89 f7                                        	mov    %r14,%rdi
   6daa9:	ff 15 21 03 20 00                               	call   *0x200321(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   6daaf:	48 81 c4 c8 00 00 00                            	add    $0xc8,%rsp
   6dab6:	5b                                              	pop    %rbx
   6dab7:	41 5c                                           	pop    %r12
   6dab9:	41 5d                                           	pop    %r13
   6dabb:	41 5e                                           	pop    %r14
   6dabd:	41 5f                                           	pop    %r15
   6dabf:	5d                                              	pop    %rbp
   6dac0:	c3                                              	ret
   6dac1:	4d 85 e4                                        	test   %r12,%r12
   6dac4:	79 0d                                           	jns    6dad3 <classic_compare_worker::encode+0x6d3>
   6dac6:	31 ff                                           	xor    %edi,%edi
   6dac8:	4c 89 e6                                        	mov    %r12,%rsi
   6dacb:	ff 15 c7 02 20 00                               	call   *0x2002c7(%rip)        # 26dd98 <_DYNAMIC+0x218>
   6dad1:	0f 0b                                           	ud2
   6dad3:	74 26                                           	je     6dafb <classic_compare_worker::encode+0x6fb>
   6dad5:	4c 89 e7                                        	mov    %r12,%rdi
   6dad8:	ff 15 0a 03 20 00                               	call   *0x20030a(%rip)        # 26dde8 <malloc@GLIBC_2.2.5>
   6dade:	48 85 c0                                        	test   %rax,%rax
   6dae1:	0f 84 99 00 00 00                               	je     6db80 <classic_compare_worker::encode+0x780>
   6dae7:	49 89 c7                                        	mov    %rax,%r15
   6daea:	48 89 c7                                        	mov    %rax,%rdi
   6daed:	48 89 de                                        	mov    %rbx,%rsi
   6daf0:	4c 89 e2                                        	mov    %r12,%rdx
   6daf3:	ff 15 bf 02 20 00                               	call   *0x2002bf(%rip)        # 26ddb8 <memcpy@GLIBC_2.14>
   6daf9:	eb 06                                           	jmp    6db01 <classic_compare_worker::encode+0x701>
   6dafb:	41 bf 01 00 00 00                               	mov    $0x1,%r15d
   6db01:	48 89 df                                        	mov    %rbx,%rdi
   6db04:	ff 15 86 04 20 00                               	call   *0x200486(%rip)        # 26df90 <_DYNAMIC+0x410>
   6db0a:	48 8b 44 24 38                                  	mov    0x38(%rsp),%rax
   6db0f:	4c 89 60 08                                     	mov    %r12,0x8(%rax)
   6db13:	4c 89 78 10                                     	mov    %r15,0x10(%rax)
   6db17:	4c 89 60 18                                     	mov    %r12,0x18(%rax)
   6db1b:	48 c7 00 00 00 00 00                            	movq   $0x0,(%rax)
   6db22:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   6db28:	74 85                                           	je     6daaf <classic_compare_worker::encode+0x6af>
   6db2a:	e9 77 ff ff ff                                  	jmp    6daa6 <classic_compare_worker::encode+0x6a6>
   6db2f:	48 83 7c 24 08 00                               	cmpq   $0x0,0x8(%rsp)
   6db35:	75 19                                           	jne    6db50 <classic_compare_worker::encode+0x750>
   6db37:	eb 23                                           	jmp    6db5c <classic_compare_worker::encode+0x75c>
   6db39:	48 83 7c 24 08 00                               	cmpq   $0x0,0x8(%rsp)
   6db3f:	48 89 de                                        	mov    %rbx,%rsi
   6db42:	75 0c                                           	jne    6db50 <classic_compare_worker::encode+0x750>
   6db44:	eb 16                                           	jmp    6db5c <classic_compare_worker::encode+0x75c>
   6db46:	48 85 c9                                        	test   %rcx,%rcx
   6db49:	74 11                                           	je     6db5c <classic_compare_worker::encode+0x75c>
   6db4b:	48 8d 44 24 08                                  	lea    0x8(%rsp),%rax
   6db50:	48 8b 38                                        	mov    (%rax),%rdi
   6db53:	ff 15 77 02 20 00                               	call   *0x200277(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   6db59:	48 89 de                                        	mov    %rbx,%rsi
   6db5c:	48 8b 84 24 c0 00 00 00                         	mov    0xc0(%rsp),%rax
   6db64:	48 89 46 18                                     	mov    %rax,0x18(%rsi)
   6db68:	0f 10 84 24 b0 00 00 00                         	movups 0xb0(%rsp),%xmm0
   6db70:	0f 11 46 08                                     	movups %xmm0,0x8(%rsi)
   6db74:	48 c7 06 01 00 00 00                            	movq   $0x1,(%rsi)
   6db7b:	e9 2f ff ff ff                                  	jmp    6daaf <classic_compare_worker::encode+0x6af>
   6db80:	bf 01 00 00 00                                  	mov    $0x1,%edi
   6db85:	e9 3e ff ff ff                                  	jmp    6dac8 <classic_compare_worker::encode+0x6c8>
   6db8a:	bf 04 00 00 00                                  	mov    $0x4,%edi
   6db8f:	48 89 de                                        	mov    %rbx,%rsi
   6db92:	ff 15 00 02 20 00                               	call   *0x200200(%rip)        # 26dd98 <_DYNAMIC+0x218>
   6db98:	48 8d 3d a1 8d 1f 00                            	lea    0x1f8da1(%rip),%rdi        # 266940 <__frame_dummy_init_array_entry+0x290>
   6db9f:	ff 15 0b 04 20 00                               	call   *0x20040b(%rip)        # 26dfb0 <_DYNAMIC+0x430>
   6dba5:	eb 13                                           	jmp    6dbba <classic_compare_worker::encode+0x7ba>
   6dba7:	48 89 c3                                        	mov    %rax,%rbx
   6dbaa:	48 89 e7                                        	mov    %rsp,%rdi
   6dbad:	e8 3e 5a ff ff                                  	call   635f0 <core::ptr::drop_glue::<emuella_j2k_core::J2kError>>
   6dbb2:	48 89 df                                        	mov    %rbx,%rdi
   6dbb5:	e8 f6 78 1f 00                                  	call   2654b0 <_Unwind_Resume@plt>
   6dbba:	48 89 c3                                        	mov    %rax,%rbx
   6dbbd:	48 89 e7                                        	mov    %rsp,%rdi
   6dbc0:	e8 ab 59 ff ff                                  	call   63570 <core::ptr::drop_glue::<emuella_j2k_core::EncodeOptions>>
   6dbc5:	48 89 df                                        	mov    %rbx,%rdi
   6dbc8:	e8 e3 78 1f 00                                  	call   2654b0 <_Unwind_Resume@plt>
   6dbcd:	48 89 c3                                        	mov    %rax,%rbx
   6dbd0:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   6dbd6:	74 09                                           	je     6dbe1 <classic_compare_worker::encode+0x7e1>
   6dbd8:	4c 89 f7                                        	mov    %r14,%rdi
   6dbdb:	ff 15 ef 01 20 00                               	call   *0x2001ef(%rip)        # 26ddd0 <free@GLIBC_2.2.5>
   6dbe1:	48 89 df                                        	mov    %rbx,%rdi
   6dbe4:	e8 c7 78 1f 00                                  	call   2654b0 <_Unwind_Resume@plt>
