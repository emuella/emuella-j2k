Disassembly of section .text:

00000000001ba5a0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>>:
  1ba5a0:	55                                              	push   %rbp
  1ba5a1:	41 57                                           	push   %r15
  1ba5a3:	41 56                                           	push   %r14
  1ba5a5:	41 55                                           	push   %r13
  1ba5a7:	41 54                                           	push   %r12
  1ba5a9:	53                                              	push   %rbx
  1ba5aa:	48 81 ec a8 00 00 00                            	sub    $0xa8,%rsp
  1ba5b1:	48 89 7c 24 18                                  	mov    %rdi,0x18(%rsp)
  1ba5b6:	48 8b 47 40                                     	mov    0x40(%rdi),%rax
  1ba5ba:	48 89 44 24 38                                  	mov    %rax,0x38(%rsp)
  1ba5bf:	48 85 c0                                        	test   %rax,%rax
  1ba5c2:	0f 84 e2 04 00 00                               	je     1baaaa <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x50a>
  1ba5c8:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
  1ba5cd:	48 8b 40 30                                     	mov    0x30(%rax),%rax
  1ba5d1:	48 89 44 24 60                                  	mov    %rax,0x60(%rsp)
  1ba5d6:	48 85 c0                                        	test   %rax,%rax
  1ba5d9:	0f 84 cb 04 00 00                               	je     1baaaa <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x50a>
  1ba5df:	48 89 f3                                        	mov    %rsi,%rbx
  1ba5e2:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  1ba5e7:	0f b6 41 48                                     	movzbl 0x48(%rcx),%eax
  1ba5eb:	83 e0 1f                                        	and    $0x1f,%eax
  1ba5ee:	89 44 24 30                                     	mov    %eax,0x30(%rsp)
  1ba5f2:	0f b6 41 49                                     	movzbl 0x49(%rcx),%eax
  1ba5f6:	48 8d 04 c0                                     	lea    (%rax,%rax,8),%rax
  1ba5fa:	48 8d 04 80                                     	lea    (%rax,%rax,4),%rax
  1ba5fe:	48 8d 15 df 7d e6 ff                            	lea    -0x198221(%rip),%rdx        # 223e4 <anon.38d8fac72642096468ff551e1950d360.1255.llvm.10326835611459877753+0x4724>
  1ba605:	48 01 c2                                        	add    %rax,%rdx
  1ba608:	48 89 94 24 90 00 00 00                         	mov    %rdx,0x90(%rsp)
  1ba610:	4c 8b 79 38                                     	mov    0x38(%rcx),%r15
  1ba614:	4c 8b 21                                        	mov    (%rcx),%r12
  1ba617:	4c 8b 71 08                                     	mov    0x8(%rcx),%r14
  1ba61b:	48 8b 41 28                                     	mov    0x28(%rcx),%rax
  1ba61f:	48 89 44 24 50                                  	mov    %rax,0x50(%rsp)
  1ba624:	48 8b 41 20                                     	mov    0x20(%rcx),%rax
  1ba628:	48 89 84 24 88 00 00 00                         	mov    %rax,0x88(%rsp)
  1ba630:	48 8b 4c 24 38                                  	mov    0x38(%rsp),%rcx
  1ba635:	48 89 ca                                        	mov    %rcx,%rdx
  1ba638:	48 c1 ea 02                                     	shr    $0x2,%rdx
  1ba63c:	89 c8                                           	mov    %ecx,%eax
  1ba63e:	83 e0 03                                        	and    $0x3,%eax
  1ba641:	48 83 f8 01                                     	cmp    $0x1,%rax
  1ba645:	48 83 da ff                                     	sbb    $0xffffffffffffffff,%rdx
  1ba649:	48 89 54 24 40                                  	mov    %rdx,0x40(%rsp)
  1ba64e:	31 d2                                           	xor    %edx,%edx
  1ba650:	4c 89 b4 24 80 00 00 00                         	mov    %r14,0x80(%rsp)
  1ba658:	4c 89 7c 24 78                                  	mov    %r15,0x78(%rsp)
  1ba65d:	4c 89 64 24 70                                  	mov    %r12,0x70(%rsp)
  1ba662:	eb 34                                           	jmp    1ba698 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0xf8>
  1ba664:	66 66 66 2e 0f 1f 84 00 00 00 00 00             	data16 data16 cs nopw 0x0(%rax,%rax,1)
  1ba670:	48 8b 54 24 48                                  	mov    0x48(%rsp),%rdx
  1ba675:	48 83 c2 04                                     	add    $0x4,%rdx
  1ba679:	48 8b 44 24 40                                  	mov    0x40(%rsp),%rax
  1ba67e:	48 ff c8                                        	dec    %rax
  1ba681:	48 8b 4c 24 58                                  	mov    0x58(%rsp),%rcx
  1ba686:	48 83 c1 fc                                     	add    $0xfffffffffffffffc,%rcx
  1ba68a:	48 89 44 24 40                                  	mov    %rax,0x40(%rsp)
  1ba68f:	48 85 c0                                        	test   %rax,%rax
  1ba692:	0f 84 12 04 00 00                               	je     1baaaa <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x50a>
  1ba698:	48 83 f9 01                                     	cmp    $0x1,%rcx
  1ba69c:	48 89 4c 24 58                                  	mov    %rcx,0x58(%rsp)
  1ba6a1:	48 89 cd                                        	mov    %rcx,%rbp
  1ba6a4:	48 83 d5 00                                     	adc    $0x0,%rbp
  1ba6a8:	48 83 fd 04                                     	cmp    $0x4,%rbp
  1ba6ac:	b8 04 00 00 00                                  	mov    $0x4,%eax
  1ba6b1:	48 0f 43 e8                                     	cmovae %rax,%rbp
  1ba6b5:	48 89 54 24 48                                  	mov    %rdx,0x48(%rsp)
  1ba6ba:	48 39 54 24 38                                  	cmp    %rdx,0x38(%rsp)
  1ba6bf:	74 af                                           	je     1ba670 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0xd0>
  1ba6c1:	48 8b 44 24 48                                  	mov    0x48(%rsp),%rax
  1ba6c6:	48 83 c8 01                                     	or     $0x1,%rax
  1ba6ca:	49 0f af c7                                     	imul   %r15,%rax
  1ba6ce:	48 ff c0                                        	inc    %rax
  1ba6d1:	48 89 44 24 68                                  	mov    %rax,0x68(%rsp)
  1ba6d6:	31 c9                                           	xor    %ecx,%ecx
  1ba6d8:	48 89 ac 24 98 00 00 00                         	mov    %rbp,0x98(%rsp)
  1ba6e0:	eb 19                                           	jmp    1ba6fb <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x15b>
  1ba6e2:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  1ba6f0:	48 3b 4c 24 60                                  	cmp    0x60(%rsp),%rcx
  1ba6f5:	0f 84 75 ff ff ff                               	je     1ba670 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0xd0>
  1ba6fb:	48 8b 44 24 68                                  	mov    0x68(%rsp),%rax
  1ba700:	48 01 c8                                        	add    %rcx,%rax
  1ba703:	48 ff c1                                        	inc    %rcx
  1ba706:	31 d2                                           	xor    %edx,%edx
  1ba708:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  1ba70d:	eb 0c                                           	jmp    1ba71b <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x17b>
  1ba70f:	90                                              	nop
  1ba710:	48 ff c2                                        	inc    %rdx
  1ba713:	4c 01 f8                                        	add    %r15,%rax
  1ba716:	48 39 ea                                        	cmp    %rbp,%rdx
  1ba719:	74 d5                                           	je     1ba6f0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x150>
  1ba71b:	4c 39 f0                                        	cmp    %r14,%rax
  1ba71e:	0f 83 98 03 00 00                               	jae    1baabc <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x51c>
  1ba724:	48 8d 34 40                                     	lea    (%rax,%rax,2),%rsi
  1ba728:	48 89 b4 24 a0 00 00 00                         	mov    %rsi,0xa0(%rsp)
  1ba730:	41 80 3c 34 00                                  	cmpb   $0x0,(%r12,%rsi,1)
  1ba735:	75 d9                                           	jne    1ba710 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x170>
  1ba737:	4c 89 e7                                        	mov    %r12,%rdi
  1ba73a:	4c 89 f6                                        	mov    %r14,%rsi
  1ba73d:	48 89 54 24 08                                  	mov    %rdx,0x8(%rsp)
  1ba742:	48 89 c2                                        	mov    %rax,%rdx
  1ba745:	4c 89 f9                                        	mov    %r15,%rcx
  1ba748:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1ba74d:	e8 7e 65 ff ff                                  	call   1b0cd0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1ba752:	89 c1                                           	mov    %eax,%ecx
  1ba754:	c1 e9 18                                        	shr    $0x18,%ecx
  1ba757:	80 e1 01                                        	and    $0x1,%cl
  1ba75a:	48 89 c2                                        	mov    %rax,%rdx
  1ba75d:	48 c1 ea 20                                     	shr    $0x20,%rdx
  1ba761:	80 e2 01                                        	and    $0x1,%dl
  1ba764:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1ba769:	10 ca                                           	adc    %cl,%dl
  1ba76b:	0f ba e0 08                                     	bt     $0x8,%eax
  1ba76f:	80 d2 00                                        	adc    $0x0,%dl
  1ba772:	48 89 c1                                        	mov    %rax,%rcx
  1ba775:	48 c1 e9 38                                     	shr    $0x38,%rcx
  1ba779:	89 c6                                           	mov    %eax,%esi
  1ba77b:	40 80 e6 01                                     	and    $0x1,%sil
  1ba77f:	89 c7                                           	mov    %eax,%edi
  1ba781:	c1 ef 10                                        	shr    $0x10,%edi
  1ba784:	40 80 e7 01                                     	and    $0x1,%dil
  1ba788:	40 00 f1                                        	add    %sil,%cl
  1ba78b:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1ba790:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1ba795:	40 10 f9                                        	adc    %dil,%cl
  1ba798:	08 d1                                           	or     %dl,%cl
  1ba79a:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1ba79f:	48 8b 4c 24 20                                  	mov    0x20(%rsp),%rcx
  1ba7a4:	0f 84 66 ff ff ff                               	je     1ba710 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x170>
  1ba7aa:	48 3b 44 24 50                                  	cmp    0x50(%rsp),%rax
  1ba7af:	0f 83 41 03 00 00                               	jae    1baaf6 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x556>
  1ba7b5:	48 89 c2                                        	mov    %rax,%rdx
  1ba7b8:	48 8b 84 24 88 00 00 00                         	mov    0x88(%rsp),%rax
  1ba7c0:	8b 04 90                                        	mov    (%rax,%rdx,4),%eax
  1ba7c3:	45 31 ed                                        	xor    %r13d,%r13d
  1ba7c6:	8b 4c 24 30                                     	mov    0x30(%rsp),%ecx
  1ba7ca:	0f a3 c8                                        	bt     %ecx,%eax
  1ba7cd:	40 0f 92 c5                                     	setb   %bpl
  1ba7d1:	4c 89 e7                                        	mov    %r12,%rdi
  1ba7d4:	4c 89 f6                                        	mov    %r14,%rsi
  1ba7d7:	4c 89 f9                                        	mov    %r15,%rcx
  1ba7da:	e8 f1 64 ff ff                                  	call   1b0cd0 <emuella_j2k_tier1::neighborhood_at::<false>>
  1ba7df:	89 c1                                           	mov    %eax,%ecx
  1ba7e1:	c1 e9 18                                        	shr    $0x18,%ecx
  1ba7e4:	83 e1 01                                        	and    $0x1,%ecx
  1ba7e7:	89 c2                                           	mov    %eax,%edx
  1ba7e9:	c1 ea 08                                        	shr    $0x8,%edx
  1ba7ec:	83 e2 01                                        	and    $0x1,%edx
  1ba7ef:	48 0f ba e0 30                                  	bt     $0x30,%rax
  1ba7f4:	48 83 d2 00                                     	adc    $0x0,%rdx
  1ba7f8:	89 c6                                           	mov    %eax,%esi
  1ba7fa:	83 e6 01                                        	and    $0x1,%esi
  1ba7fd:	89 c7                                           	mov    %eax,%edi
  1ba7ff:	c1 ef 10                                        	shr    $0x10,%edi
  1ba802:	83 e7 01                                        	and    $0x1,%edi
  1ba805:	49 89 c0                                        	mov    %rax,%r8
  1ba808:	49 c1 e8 38                                     	shr    $0x38,%r8
  1ba80c:	49 01 f0                                        	add    %rsi,%r8
  1ba80f:	48 0f ba e0 28                                  	bt     $0x28,%rax
  1ba814:	49 11 f8                                        	adc    %rdi,%r8
  1ba817:	48 8d 14 52                                     	lea    (%rdx,%rdx,2),%rdx
  1ba81b:	48 0f ba e0 20                                  	bt     $0x20,%rax
  1ba820:	48 13 8c 24 90 00 00 00                         	adc    0x90(%rsp),%rcx
  1ba828:	4b 8d 04 c0                                     	lea    (%r8,%r8,8),%rax
  1ba82c:	48 01 d1                                        	add    %rdx,%rcx
  1ba82f:	0f b6 3c 01                                     	movzbl (%rcx,%rax,1),%edi
  1ba833:	48 83 ff 12                                     	cmp    $0x12,%rdi
  1ba837:	0f 87 92 02 00 00                               	ja     1baacf <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x52f>
  1ba83d:	0f b6 44 7b 1c                                  	movzbl 0x1c(%rbx,%rdi,2),%eax
  1ba842:	48 83 f8 2e                                     	cmp    $0x2e,%rax
  1ba846:	0f 87 95 02 00 00                               	ja     1baae1 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x541>
  1ba84c:	41 88 ed                                        	mov    %bpl,%r13b
  1ba84f:	0f b6 4c 7b 1d                                  	movzbl 0x1d(%rbx,%rdi,2),%ecx
  1ba854:	4c 8d 05 35 81 e6 ff                            	lea    -0x197ecb(%rip),%r8        # 22990 <anon.36ada1a7bb6c1d545348b8e8a90a5caf.101.llvm.17933956792295344108+0x31>
  1ba85b:	41 8b 2c c0                                     	mov    (%r8,%rax,8),%ebp
  1ba85f:	41 0f b6 54 c0 04                               	movzbl 0x4(%r8,%rax,8),%edx
  1ba865:	41 0f b6 74 c0 05                               	movzbl 0x5(%r8,%rax,8),%esi
  1ba86b:	45 0f b6 44 c0 06                               	movzbl 0x6(%r8,%rax,8),%r8d
  1ba871:	8b 43 10                                        	mov    0x10(%rbx),%eax
  1ba874:	29 e8                                           	sub    %ebp,%eax
  1ba876:	89 43 10                                        	mov    %eax,0x10(%rbx)
  1ba879:	41 39 cd                                        	cmp    %ecx,%r13d
  1ba87c:	44 89 6c 24 34                                  	mov    %r13d,0x34(%rsp)
  1ba881:	75 10                                           	jne    1ba893 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x2f3>
  1ba883:	66 85 c0                                        	test   %ax,%ax
  1ba886:	78 25                                           	js     1ba8ad <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x30d>
  1ba888:	39 e8                                           	cmp    %ebp,%eax
  1ba88a:	73 51                                           	jae    1ba8dd <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x33d>
  1ba88c:	89 6b 10                                        	mov    %ebp,0x10(%rbx)
  1ba88f:	89 e8                                           	mov    %ebp,%eax
  1ba891:	eb 4d                                           	jmp    1ba8e0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x340>
  1ba893:	39 e8                                           	cmp    %ebp,%eax
  1ba895:	73 23                                           	jae    1ba8ba <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x31a>
  1ba897:	01 6b 14                                        	add    %ebp,0x14(%rbx)
  1ba89a:	89 c5                                           	mov    %eax,%ebp
  1ba89c:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1ba8a1:	40 88 74 7b 1c                                  	mov    %sil,0x1c(%rbx,%rdi,2)
  1ba8a6:	45 84 c0                                        	test   %r8b,%r8b
  1ba8a9:	75 21                                           	jne    1ba8cc <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x32c>
  1ba8ab:	eb 26                                           	jmp    1ba8d3 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x333>
  1ba8ad:	01 6b 14                                        	add    %ebp,0x14(%rbx)
  1ba8b0:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1ba8b5:	e9 86 01 00 00                                  	jmp    1baa40 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x4a0>
  1ba8ba:	89 6b 10                                        	mov    %ebp,0x10(%rbx)
  1ba8bd:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1ba8c2:	40 88 74 7b 1c                                  	mov    %sil,0x1c(%rbx,%rdi,2)
  1ba8c7:	45 84 c0                                        	test   %r8b,%r8b
  1ba8ca:	74 07                                           	je     1ba8d3 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x333>
  1ba8cc:	80 f1 01                                        	xor    $0x1,%cl
  1ba8cf:	88 4c 7b 1d                                     	mov    %cl,0x1d(%rbx,%rdi,2)
  1ba8d3:	66 85 ed                                        	test   %bp,%bp
  1ba8d6:	79 1c                                           	jns    1ba8f4 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x354>
  1ba8d8:	e9 63 01 00 00                                  	jmp    1baa40 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x4a0>
  1ba8dd:	01 6b 14                                        	add    %ebp,0x14(%rbx)
  1ba8e0:	88 54 7b 1c                                     	mov    %dl,0x1c(%rbx,%rdi,2)
  1ba8e4:	89 c5                                           	mov    %eax,%ebp
  1ba8e6:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1ba8eb:	66 85 ed                                        	test   %bp,%bp
  1ba8ee:	0f 88 4c 01 00 00                               	js     1baa40 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x4a0>
  1ba8f4:	44 8b 6b 14                                     	mov    0x14(%rbx),%r13d
  1ba8f8:	8b 43 18                                        	mov    0x18(%rbx),%eax
  1ba8fb:	48 8b 73 08                                     	mov    0x8(%rbx),%rsi
  1ba8ff:	0f b6 4b 45                                     	movzbl 0x45(%rbx),%ecx
  1ba903:	44 0f b6 63 46                                  	movzbl 0x46(%rbx),%r12d
  1ba908:	48 89 74 24 10                                  	mov    %rsi,0x10(%rsp)
  1ba90d:	eb 43                                           	jmp    1ba952 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x3b2>
  1ba90f:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1ba913:	46 88 24 30                                     	mov    %r12b,(%rax,%r14,1)
  1ba917:	49 ff c6                                        	inc    %r14
  1ba91a:	4c 89 76 10                                     	mov    %r14,0x10(%rsi)
  1ba91e:	45 89 ec                                        	mov    %r13d,%r12d
  1ba921:	41 c1 ec 13                                     	shr    $0x13,%r12d
  1ba925:	b8 08 00 00 00                                  	mov    $0x8,%eax
  1ba92a:	b9 fe ff 07 00                                  	mov    $0x7fffe,%ecx
  1ba92f:	c6 43 45 01                                     	movb   $0x1,0x45(%rbx)
  1ba933:	44 88 63 46                                     	mov    %r12b,0x46(%rbx)
  1ba937:	41 21 cd                                        	and    %ecx,%r13d
  1ba93a:	44 89 6b 14                                     	mov    %r13d,0x14(%rbx)
  1ba93e:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1ba941:	b1 01                                           	mov    $0x1,%cl
  1ba943:	f7 c5 00 40 00 00                               	test   $0x4000,%ebp
  1ba949:	44 89 fd                                        	mov    %r15d,%ebp
  1ba94c:	0f 85 ee 00 00 00                               	jne    1baa40 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x4a0>
  1ba952:	44 8d 3c 6d 00 00 00 00                         	lea    0x0(,%rbp,2),%r15d
  1ba95a:	44 89 7b 10                                     	mov    %r15d,0x10(%rbx)
  1ba95e:	45 01 ed                                        	add    %r13d,%r13d
  1ba961:	44 89 6b 14                                     	mov    %r13d,0x14(%rbx)
  1ba965:	ff c8                                           	dec    %eax
  1ba967:	89 43 18                                        	mov    %eax,0x18(%rbx)
  1ba96a:	75 d7                                           	jne    1ba943 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x3a3>
  1ba96c:	c6 43 45 00                                     	movb   $0x0,0x45(%rbx)
  1ba970:	80 f9 01                                        	cmp    $0x1,%cl
  1ba973:	75 a9                                           	jne    1ba91e <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x37e>
  1ba975:	41 80 fc ff                                     	cmp    $0xff,%r12b
  1ba979:	74 58                                           	je     1ba9d3 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x433>
  1ba97b:	41 81 fd ff ff ff 07                            	cmp    $0x7ffffff,%r13d
  1ba982:	0f 86 8f 00 00 00                               	jbe    1baa17 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x477>
  1ba988:	41 fe c4                                        	inc    %r12b
  1ba98b:	4c 8b 76 10                                     	mov    0x10(%rsi),%r14
  1ba98f:	41 80 fc ff                                     	cmp    $0xff,%r12b
  1ba993:	0f 85 82 00 00 00                               	jne    1baa1b <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x47b>
  1ba999:	45 89 ec                                        	mov    %r13d,%r12d
  1ba99c:	41 81 e4 fe ff ff 07                            	and    $0x7fffffe,%r12d
  1ba9a3:	44 89 63 14                                     	mov    %r12d,0x14(%rbx)
  1ba9a7:	4c 3b 36                                        	cmp    (%rsi),%r14
  1ba9aa:	75 15                                           	jne    1ba9c1 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x421>
  1ba9ac:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1ba9b1:	ff 15 89 a4 0b 00                               	call   *0xba489(%rip)        # 274e40 <_DYNAMIC+0x290>
  1ba9b7:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1ba9bc:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1ba9c1:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1ba9c5:	42 c6 04 30 ff                                  	movb   $0xff,(%rax,%r14,1)
  1ba9ca:	49 ff c6                                        	inc    %r14
  1ba9cd:	4c 89 76 10                                     	mov    %r14,0x10(%rsi)
  1ba9d1:	eb 31                                           	jmp    1baa04 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x464>
  1ba9d3:	4c 8b 76 10                                     	mov    0x10(%rsi),%r14
  1ba9d7:	4c 3b 36                                        	cmp    (%rsi),%r14
  1ba9da:	75 15                                           	jne    1ba9f1 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x451>
  1ba9dc:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1ba9e1:	ff 15 59 a4 0b 00                               	call   *0xba459(%rip)        # 274e40 <_DYNAMIC+0x290>
  1ba9e7:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1ba9ec:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1ba9f1:	48 8b 46 08                                     	mov    0x8(%rsi),%rax
  1ba9f5:	42 c6 04 30 ff                                  	movb   $0xff,(%rax,%r14,1)
  1ba9fa:	49 ff c6                                        	inc    %r14
  1ba9fd:	4c 89 76 10                                     	mov    %r14,0x10(%rsi)
  1baa01:	45 89 ec                                        	mov    %r13d,%r12d
  1baa04:	41 c1 ec 14                                     	shr    $0x14,%r12d
  1baa08:	b8 07 00 00 00                                  	mov    $0x7,%eax
  1baa0d:	b9 fe ff 0f 00                                  	mov    $0xffffe,%ecx
  1baa12:	e9 18 ff ff ff                                  	jmp    1ba92f <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x38f>
  1baa17:	4c 8b 76 10                                     	mov    0x10(%rsi),%r14
  1baa1b:	4c 3b 36                                        	cmp    (%rsi),%r14
  1baa1e:	0f 85 eb fe ff ff                               	jne    1ba90f <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x36f>
  1baa24:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1baa29:	ff 15 11 a4 0b 00                               	call   *0xba411(%rip)        # 274e40 <_DYNAMIC+0x290>
  1baa2f:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1baa34:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1baa39:	e9 d1 fe ff ff                                  	jmp    1ba90f <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x36f>
  1baa3e:	66 90                                           	xchg   %ax,%ax
  1baa40:	4c 8b 64 24 70                                  	mov    0x70(%rsp),%r12
  1baa45:	48 8b 84 24 a0 00 00 00                         	mov    0xa0(%rsp),%rax
  1baa4d:	4c 01 e0                                        	add    %r12,%rax
  1baa50:	49 89 c5                                        	mov    %rax,%r13
  1baa53:	c6 40 01 01                                     	movb   $0x1,0x1(%rax)
  1baa57:	83 7c 24 34 00                                  	cmpl   $0x0,0x34(%rsp)
  1baa5c:	4c 8b b4 24 80 00 00 00                         	mov    0x80(%rsp),%r14
  1baa64:	4c 8b 7c 24 78                                  	mov    0x78(%rsp),%r15
  1baa69:	48 8b ac 24 98 00 00 00                         	mov    0x98(%rsp),%rbp
  1baa71:	48 8b 4c 24 20                                  	mov    0x20(%rsp),%rcx
  1baa76:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1baa7b:	0f 84 8f fc ff ff                               	je     1ba710 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x170>
  1baa81:	48 89 c7                                        	mov    %rax,%rdi
  1baa84:	48 8b 74 24 18                                  	mov    0x18(%rsp),%rsi
  1baa89:	48 89 da                                        	mov    %rbx,%rdx
  1baa8c:	e8 8f 65 ff ff                                  	call   1b1020 <emuella_j2k_tier1::encode_sign_bit_at::<false>>
  1baa91:	48 8b 54 24 08                                  	mov    0x8(%rsp),%rdx
  1baa96:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1baa9b:	48 8b 4c 24 20                                  	mov    0x20(%rsp),%rcx
  1baaa0:	41 c6 45 00 01                                  	movb   $0x1,0x0(%r13)
  1baaa5:	e9 66 fc ff ff                                  	jmp    1ba710 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>+0x170>
  1baaaa:	48 81 c4 a8 00 00 00                            	add    $0xa8,%rsp
  1baab1:	5b                                              	pop    %rbx
  1baab2:	41 5c                                           	pop    %r12
  1baab4:	41 5d                                           	pop    %r13
  1baab6:	41 5e                                           	pop    %r14
  1baab8:	41 5f                                           	pop    %r15
  1baaba:	5d                                              	pop    %rbp
  1baabb:	c3                                              	ret
  1baabc:	48 8d 15 3d 5a 0b 00                            	lea    0xb5a3d(%rip),%rdx        # 270500 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x2888>
  1baac3:	48 89 c7                                        	mov    %rax,%rdi
  1baac6:	4c 89 f6                                        	mov    %r14,%rsi
  1baac9:	ff 15 41 a3 0b 00                               	call   *0xba341(%rip)        # 274e10 <_DYNAMIC+0x260>
  1baacf:	48 8d 15 da 61 0b 00                            	lea    0xb61da(%rip),%rdx        # 270cb0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3038>
  1baad6:	be 13 00 00 00                                  	mov    $0x13,%esi
  1baadb:	ff 15 2f a3 0b 00                               	call   *0xba32f(%rip)        # 274e10 <_DYNAMIC+0x260>
  1baae1:	48 8d 15 e0 61 0b 00                            	lea    0xb61e0(%rip),%rdx        # 270cc8 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x3050>
  1baae8:	be 2f 00 00 00                                  	mov    $0x2f,%esi
  1baaed:	48 89 c7                                        	mov    %rax,%rdi
  1baaf0:	ff 15 1a a3 0b 00                               	call   *0xba31a(%rip)        # 274e10 <_DYNAMIC+0x260>
  1baaf6:	48 8d 15 8b 59 0b 00                            	lea    0xb598b(%rip),%rdx        # 270488 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x2810>
  1baafd:	48 89 c7                                        	mov    %rax,%rdi
  1bab00:	48 8b 74 24 50                                  	mov    0x50(%rsp),%rsi
  1bab05:	ff 15 05 a3 0b 00                               	call   *0xba305(%rip)        # 274e10 <_DYNAMIC+0x260>
