Disassembly of section .text:

00000000002013c0 <emuella_j2k_transform::forward_reversible_5_3_bounded>:
  2013c0:	55                                              	push   %rbp
  2013c1:	41 57                                           	push   %r15
  2013c3:	41 56                                           	push   %r14
  2013c5:	41 55                                           	push   %r13
  2013c7:	41 54                                           	push   %r12
  2013c9:	53                                              	push   %rbx
  2013ca:	48 83 ec 68                                     	sub    $0x68,%rsp
  2013ce:	4c 8b 39                                        	mov    (%rcx),%r15
  2013d1:	45 31 d2                                        	xor    %r10d,%r10d
  2013d4:	4d 85 ff                                        	test   %r15,%r15
  2013d7:	74 18                                           	je     2013f1 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x31>
  2013d9:	4c 8b 71 08                                     	mov    0x8(%rcx),%r14
  2013dd:	4d 85 f6                                        	test   %r14,%r14
  2013e0:	74 0f                                           	je     2013f1 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x31>
  2013e2:	48 8b 59 10                                     	mov    0x10(%rcx),%rbx
  2013e6:	4c 39 fb                                        	cmp    %r15,%rbx
  2013e9:	73 23                                           	jae    20140e <emuella_j2k_transform::forward_reversible_5_3_bounded+0x4e>
  2013eb:	41 ba 01 00 00 00                               	mov    $0x1,%r10d
  2013f1:	44 89 17                                        	mov    %r10d,(%rdi)
  2013f4:	48 89 47 08                                     	mov    %rax,0x8(%rdi)
  2013f8:	4c 89 4f 10                                     	mov    %r9,0x10(%rdi)
  2013fc:	48 89 f8                                        	mov    %rdi,%rax
  2013ff:	48 83 c4 68                                     	add    $0x68,%rsp
  201403:	5b                                              	pop    %rbx
  201404:	41 5c                                           	pop    %r12
  201406:	41 5d                                           	pop    %r13
  201408:	41 5e                                           	pop    %r14
  20140a:	41 5f                                           	pop    %r15
  20140c:	5d                                              	pop    %rbp
  20140d:	c3                                              	ret
  20140e:	48 89 54 24 10                                  	mov    %rdx,0x10(%rsp)
  201413:	49 8d 46 ff                                     	lea    -0x1(%r14),%rax
  201417:	48 f7 e3                                        	mul    %rbx
  20141a:	41 ba 02 00 00 00                               	mov    $0x2,%r10d
  201420:	70 cf                                           	jo     2013f1 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x31>
  201422:	4c 01 f8                                        	add    %r15,%rax
  201425:	41 0f 92 c3                                     	setb   %r11b
  201429:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  20142e:	48 39 c2                                        	cmp    %rax,%rdx
  201431:	0f 92 c0                                        	setb   %al
  201434:	44 08 d8                                        	or     %r11b,%al
  201437:	75 b8                                           	jne    2013f1 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x31>
  201439:	4d 39 f7                                        	cmp    %r14,%r15
  20143c:	4d 89 f4                                        	mov    %r14,%r12
  20143f:	4d 0f 47 e7                                     	cmova  %r15,%r12
  201443:	4b 8d 04 64                                     	lea    (%r12,%r12,2),%rax
  201447:	41 ba 03 00 00 00                               	mov    $0x3,%r10d
  20144d:	49 39 c1                                        	cmp    %rax,%r9
  201450:	72 9f                                           	jb     2013f1 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x31>
  201452:	4c 8b 69 18                                     	mov    0x18(%rcx),%r13
  201456:	0f b6 69 28                                     	movzbl 0x28(%rcx),%ebp
  20145a:	4c 89 f8                                        	mov    %r15,%rax
  20145d:	48 d1 e8                                        	shr    $1,%rax
  201460:	4d 89 fb                                        	mov    %r15,%r11
  201463:	49 29 c3                                        	sub    %rax,%r11
  201466:	40 84 ed                                        	test   %bpl,%bpl
  201469:	4c 0f 45 d8                                     	cmovne %rax,%r11
  20146d:	41 ba 04 00 00 00                               	mov    $0x4,%r10d
  201473:	4d 39 dd                                        	cmp    %r11,%r13
  201476:	0f 85 75 ff ff ff                               	jne    2013f1 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x31>
  20147c:	40 88 6c 24 0f                                  	mov    %bpl,0xf(%rsp)
  201481:	4c 89 6c 24 48                                  	mov    %r13,0x48(%rsp)
  201486:	4c 8b 69 20                                     	mov    0x20(%rcx),%r13
  20148a:	0f b6 69 29                                     	movzbl 0x29(%rcx),%ebp
  20148e:	4c 89 f0                                        	mov    %r14,%rax
  201491:	48 d1 e8                                        	shr    $1,%rax
  201494:	4d 89 f3                                        	mov    %r14,%r11
  201497:	49 29 c3                                        	sub    %rax,%r11
  20149a:	40 88 6c 24 28                                  	mov    %bpl,0x28(%rsp)
  20149f:	40 84 ed                                        	test   %bpl,%bpl
  2014a2:	4c 0f 45 d8                                     	cmovne %rax,%r11
  2014a6:	4c 89 6c 24 18                                  	mov    %r13,0x18(%rsp)
  2014ab:	4d 39 dd                                        	cmp    %r11,%r13
  2014ae:	0f 85 3d ff ff ff                               	jne    2013f1 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x31>
  2014b4:	0f b6 41 31                                     	movzbl 0x31(%rcx),%eax
  2014b8:	80 79 30 00                                     	cmpb   $0x0,0x30(%rcx)
  2014bc:	74 11                                           	je     2014cf <emuella_j2k_transform::forward_reversible_5_3_bounded+0x10f>
  2014be:	04 df                                           	add    $0xdf,%al
  2014c0:	3c df                                           	cmp    $0xdf,%al
  2014c2:	77 11                                           	ja     2014d5 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x115>
  2014c4:	41 ba 05 00 00 00                               	mov    $0x5,%r10d
  2014ca:	e9 22 ff ff ff                                  	jmp    2013f1 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x31>
  2014cf:	04 e0                                           	add    $0xe0,%al
  2014d1:	3c e1                                           	cmp    $0xe1,%al
  2014d3:	72 ef                                           	jb     2014c4 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x104>
  2014d5:	4d 89 c5                                        	mov    %r8,%r13
  2014d8:	4c 89 cd                                        	mov    %r9,%rbp
  2014db:	48 89 7c 24 20                                  	mov    %rdi,0x20(%rsp)
  2014e0:	48 8d 7c 24 50                                  	lea    0x50(%rsp),%rdi
  2014e5:	48 89 74 24 30                                  	mov    %rsi,0x30(%rsp)
  2014ea:	e8 41 fd ff ff                                  	call   201230 <emuella_j2k_transform::validate_samples_in_range>
  2014ef:	83 7c 24 50 ff                                  	cmpl   $0xffffffff,0x50(%rsp)
  2014f4:	74 1b                                           	je     201511 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x151>
  2014f6:	48 8b 44 24 60                                  	mov    0x60(%rsp),%rax
  2014fb:	48 8b 7c 24 20                                  	mov    0x20(%rsp),%rdi
  201500:	48 89 47 10                                     	mov    %rax,0x10(%rdi)
  201504:	0f 10 44 24 50                                  	movups 0x50(%rsp),%xmm0
  201509:	0f 11 07                                        	movups %xmm0,(%rdi)
  20150c:	e9 eb fe ff ff                                  	jmp    2013fc <emuella_j2k_transform::forward_reversible_5_3_bounded+0x3c>
  201511:	48 89 e9                                        	mov    %rbp,%rcx
  201514:	4c 29 e1                                        	sub    %r12,%rcx
  201517:	48 8b 7c 24 20                                  	mov    0x20(%rsp),%rdi
  20151c:	0f 82 50 03 00 00                               	jb     201872 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x4b2>
  201522:	4c 29 e1                                        	sub    %r12,%rcx
  201525:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  20152a:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  20152f:	0f 82 56 03 00 00                               	jb     20188b <emuella_j2k_transform::forward_reversible_5_3_bounded+0x4cb>
  201535:	4a 8d 2c a5 00 00 00 00                         	lea    0x0(,%r12,4),%rbp
  20153d:	4c 01 ed                                        	add    %r13,%rbp
  201540:	4e 8d 2c b5 00 00 00 00                         	lea    0x0(,%r14,4),%r13
  201548:	49 39 ce                                        	cmp    %rcx,%r14
  20154b:	48 89 4c 24 38                                  	mov    %rcx,0x38(%rsp)
  201550:	0f 87 4e 03 00 00                               	ja     2018a4 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x4e4>
  201556:	4e 8d 04 a5 00 00 00 00                         	lea    0x0(,%r12,4),%r8
  20155e:	49 01 e8                                        	add    %rbp,%r8
  201561:	49 83 fe 01                                     	cmp    $0x1,%r14
  201565:	4c 89 44 24 40                                  	mov    %r8,0x40(%rsp)
  20156a:	75 1c                                           	jne    201588 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x1c8>
  20156c:	49 8d 77 ff                                     	lea    -0x1(%r15),%rsi
  201570:	48 39 f2                                        	cmp    %rsi,%rdx
  201573:	48 0f 42 f2                                     	cmovb  %rdx,%rsi
  201577:	48 83 fe 08                                     	cmp    $0x8,%rsi
  20157b:	0f 83 b2 00 00 00                               	jae    201633 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x273>
  201581:	31 f6                                           	xor    %esi,%esi
  201583:	e9 da 00 00 00                                  	jmp    201662 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x2a2>
  201588:	80 7c 24 28 00                                  	cmpb   $0x0,0x28(%rsp)
  20158d:	0f 84 ed 00 00 00                               	je     201680 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x2c0>
  201593:	45 31 e4                                        	xor    %r12d,%r12d
  201596:	4d 8d 4c 24 01                                  	lea    0x1(%r12),%r9
  20159b:	31 c9                                           	xor    %ecx,%ecx
  20159d:	4c 89 e7                                        	mov    %r12,%rdi
  2015a0:	48 39 d7                                        	cmp    %rdx,%rdi
  2015a3:	0f 83 4e 03 00 00                               	jae    2018f7 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x537>
  2015a9:	8b 34 b8                                        	mov    (%rax,%rdi,4),%esi
  2015ac:	89 74 0d 00                                     	mov    %esi,0x0(%rbp,%rcx,1)
  2015b0:	48 01 df                                        	add    %rbx,%rdi
  2015b3:	48 83 c1 04                                     	add    $0x4,%rcx
  2015b7:	49 39 cd                                        	cmp    %rcx,%r13
  2015ba:	75 e4                                           	jne    2015a0 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x1e0>
  2015bc:	4c 89 4c 24 28                                  	mov    %r9,0x28(%rsp)
  2015c1:	4c 89 f7                                        	mov    %r14,%rdi
  2015c4:	48 8b 74 24 18                                  	mov    0x18(%rsp),%rsi
  2015c9:	48 89 ea                                        	mov    %rbp,%rdx
  2015cc:	4c 89 f1                                        	mov    %r14,%rcx
  2015cf:	4d 89 f1                                        	mov    %r14,%r9
  2015d2:	e8 79 17 00 00                                  	call   202d50 <emuella_j2k_transform::transform_line_forward_first_high_bounded>
  2015d7:	4c 8b 44 24 40                                  	mov    0x40(%rsp),%r8
  2015dc:	48 8b 4c 24 38                                  	mov    0x38(%rsp),%rcx
  2015e1:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  2015e6:	48 8b 7c 24 20                                  	mov    0x20(%rsp),%rdi
  2015eb:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  2015f0:	31 f6                                           	xor    %esi,%esi
  2015f2:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  201600:	49 39 d4                                        	cmp    %rdx,%r12
  201603:	0f 83 d6 02 00 00                               	jae    2018df <emuella_j2k_transform::forward_reversible_5_3_bounded+0x51f>
  201609:	45 8b 0c 30                                     	mov    (%r8,%rsi,1),%r9d
  20160d:	46 89 0c a0                                     	mov    %r9d,(%rax,%r12,4)
  201611:	49 01 dc                                        	add    %rbx,%r12
  201614:	48 83 c6 04                                     	add    $0x4,%rsi
  201618:	49 39 f5                                        	cmp    %rsi,%r13
  20161b:	75 e3                                           	jne    201600 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x240>
  20161d:	48 8b 74 24 28                                  	mov    0x28(%rsp),%rsi
  201622:	49 89 f4                                        	mov    %rsi,%r12
  201625:	4c 39 fe                                        	cmp    %r15,%rsi
  201628:	0f 85 68 ff ff ff                               	jne    201596 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x1d6>
  20162e:	e9 eb 00 00 00                                  	jmp    20171e <emuella_j2k_transform::forward_reversible_5_3_bounded+0x35e>
  201633:	48 ff c6                                        	inc    %rsi
  201636:	41 89 f0                                        	mov    %esi,%r8d
  201639:	41 83 e0 07                                     	and    $0x7,%r8d
  20163d:	41 b9 08 00 00 00                               	mov    $0x8,%r9d
  201643:	4d 0f 45 c8                                     	cmovne %r8,%r9
  201647:	4c 29 ce                                        	sub    %r9,%rsi
  20164a:	45 31 c0                                        	xor    %r8d,%r8d
  20164d:	4d 89 c1                                        	mov    %r8,%r9
  201650:	49 83 c0 08                                     	add    $0x8,%r8
  201654:	4c 39 c6                                        	cmp    %r8,%rsi
  201657:	75 f4                                           	jne    20164d <emuella_j2k_transform::forward_reversible_5_3_bounded+0x28d>
  201659:	46 8b 44 88 1c                                  	mov    0x1c(%rax,%r9,4),%r8d
  20165e:	44 89 45 00                                     	mov    %r8d,0x0(%rbp)
  201662:	48 39 f2                                        	cmp    %rsi,%rdx
  201665:	0f 84 89 02 00 00                               	je     2018f4 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x534>
  20166b:	44 8b 04 b0                                     	mov    (%rax,%rsi,4),%r8d
  20166f:	48 ff c6                                        	inc    %rsi
  201672:	44 89 45 00                                     	mov    %r8d,0x0(%rbp)
  201676:	49 39 f7                                        	cmp    %rsi,%r15
  201679:	75 e7                                           	jne    201662 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x2a2>
  20167b:	e9 9e 00 00 00                                  	jmp    20171e <emuella_j2k_transform::forward_reversible_5_3_bounded+0x35e>
  201680:	45 31 e4                                        	xor    %r12d,%r12d
  201683:	4d 8d 4c 24 01                                  	lea    0x1(%r12),%r9
  201688:	31 c9                                           	xor    %ecx,%ecx
  20168a:	4c 89 e7                                        	mov    %r12,%rdi
  20168d:	0f 1f 00                                        	nopl   (%rax)
  201690:	48 39 d7                                        	cmp    %rdx,%rdi
  201693:	0f 83 5e 02 00 00                               	jae    2018f7 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x537>
  201699:	8b 34 b8                                        	mov    (%rax,%rdi,4),%esi
  20169c:	89 74 0d 00                                     	mov    %esi,0x0(%rbp,%rcx,1)
  2016a0:	48 01 df                                        	add    %rbx,%rdi
  2016a3:	48 83 c1 04                                     	add    $0x4,%rcx
  2016a7:	49 39 cd                                        	cmp    %rcx,%r13
  2016aa:	75 e4                                           	jne    201690 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x2d0>
  2016ac:	4c 89 4c 24 28                                  	mov    %r9,0x28(%rsp)
  2016b1:	4c 89 f7                                        	mov    %r14,%rdi
  2016b4:	48 8b 74 24 18                                  	mov    0x18(%rsp),%rsi
  2016b9:	48 89 ea                                        	mov    %rbp,%rdx
  2016bc:	4c 89 f1                                        	mov    %r14,%rcx
  2016bf:	4d 89 f1                                        	mov    %r14,%r9
  2016c2:	e8 09 0b 00 00                                  	call   2021d0 <emuella_j2k_transform::transform_line_forward_first_low_bounded>
  2016c7:	4c 8b 44 24 40                                  	mov    0x40(%rsp),%r8
  2016cc:	48 8b 4c 24 38                                  	mov    0x38(%rsp),%rcx
  2016d1:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  2016d6:	48 8b 7c 24 20                                  	mov    0x20(%rsp),%rdi
  2016db:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  2016e0:	31 f6                                           	xor    %esi,%esi
  2016e2:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  2016f0:	49 39 d4                                        	cmp    %rdx,%r12
  2016f3:	0f 83 e6 01 00 00                               	jae    2018df <emuella_j2k_transform::forward_reversible_5_3_bounded+0x51f>
  2016f9:	45 8b 0c 30                                     	mov    (%r8,%rsi,1),%r9d
  2016fd:	46 89 0c a0                                     	mov    %r9d,(%rax,%r12,4)
  201701:	49 01 dc                                        	add    %rbx,%r12
  201704:	48 83 c6 04                                     	add    $0x4,%rsi
  201708:	49 39 f5                                        	cmp    %rsi,%r13
  20170b:	75 e3                                           	jne    2016f0 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x330>
  20170d:	48 8b 74 24 28                                  	mov    0x28(%rsp),%rsi
  201712:	49 89 f4                                        	mov    %rsi,%r12
  201715:	4c 39 fe                                        	cmp    %r15,%rsi
  201718:	0f 85 65 ff ff ff                               	jne    201683 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x2c3>
  20171e:	49 39 cf                                        	cmp    %rcx,%r15
  201721:	0f 87 2b 01 00 00                               	ja     201852 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x492>
  201727:	49 83 ff 01                                     	cmp    $0x1,%r15
  20172b:	0f b6 44 24 0f                                  	movzbl 0xf(%rsp),%eax
  201730:	75 1f                                           	jne    201751 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x391>
  201732:	be 01 00 00 00                                  	mov    $0x1,%esi
  201737:	4c 8d 66 ff                                     	lea    -0x1(%rsi),%r12
  20173b:	49 39 d4                                        	cmp    %rdx,%r12
  20173e:	0f 83 d0 01 00 00                               	jae    201914 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x554>
  201744:	48 01 de                                        	add    %rbx,%rsi
  201747:	49 ff ce                                        	dec    %r14
  20174a:	75 eb                                           	jne    201737 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x377>
  20174c:	e9 f6 00 00 00                                  	jmp    201847 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x487>
  201751:	45 31 e4                                        	xor    %r12d,%r12d
  201754:	45 31 ed                                        	xor    %r13d,%r13d
  201757:	84 c0                                           	test   %al,%al
  201759:	74 77                                           	je     2017d2 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x412>
  20175b:	48 8b 6c 24 40                                  	mov    0x40(%rsp),%rbp
  201760:	4c 89 fe                                        	mov    %r15,%rsi
  201763:	4c 01 e6                                        	add    %r12,%rsi
  201766:	0f 82 a8 01 00 00                               	jb     201914 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x554>
  20176c:	48 39 d6                                        	cmp    %rdx,%rsi
  20176f:	0f 87 9f 01 00 00                               	ja     201914 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x554>
  201775:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  20177a:	4a 8d 0c a0                                     	lea    (%rax,%r12,4),%rcx
  20177e:	48 89 4c 24 18                                  	mov    %rcx,0x18(%rsp)
  201783:	4a 8d 14 28                                     	lea    (%rax,%r13,1),%rdx
  201787:	4c 89 ff                                        	mov    %r15,%rdi
  20178a:	48 8b 74 24 48                                  	mov    0x48(%rsp),%rsi
  20178f:	4c 89 f9                                        	mov    %r15,%rcx
  201792:	49 89 e8                                        	mov    %rbp,%r8
  201795:	4d 89 f9                                        	mov    %r15,%r9
  201798:	e8 b3 15 00 00                                  	call   202d50 <emuella_j2k_transform::transform_line_forward_first_high_bounded>
  20179d:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
  2017a2:	48 89 ee                                        	mov    %rbp,%rsi
  2017a5:	4a 8d 14 bd 00 00 00 00                         	lea    0x0(,%r15,4),%rdx
  2017ad:	ff 15 05 c6 06 00                               	call   *0x6c605(%rip)        # 26ddb8 <memcpy@GLIBC_2.14>
  2017b3:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  2017b8:	49 01 dc                                        	add    %rbx,%r12
  2017bb:	48 8d 04 9d 00 00 00 00                         	lea    0x0(,%rbx,4),%rax
  2017c3:	49 01 c5                                        	add    %rax,%r13
  2017c6:	49 ff ce                                        	dec    %r14
  2017c9:	48 8b 7c 24 20                                  	mov    0x20(%rsp),%rdi
  2017ce:	75 90                                           	jne    201760 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x3a0>
  2017d0:	eb 75                                           	jmp    201847 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x487>
  2017d2:	48 8b 6c 24 40                                  	mov    0x40(%rsp),%rbp
  2017d7:	4c 89 fe                                        	mov    %r15,%rsi
  2017da:	4c 01 e6                                        	add    %r12,%rsi
  2017dd:	0f 82 31 01 00 00                               	jb     201914 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x554>
  2017e3:	48 39 d6                                        	cmp    %rdx,%rsi
  2017e6:	0f 87 28 01 00 00                               	ja     201914 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x554>
  2017ec:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  2017f1:	4a 8d 0c a0                                     	lea    (%rax,%r12,4),%rcx
  2017f5:	48 89 4c 24 18                                  	mov    %rcx,0x18(%rsp)
  2017fa:	4a 8d 14 28                                     	lea    (%rax,%r13,1),%rdx
  2017fe:	4c 89 ff                                        	mov    %r15,%rdi
  201801:	48 8b 74 24 48                                  	mov    0x48(%rsp),%rsi
  201806:	4c 89 f9                                        	mov    %r15,%rcx
  201809:	49 89 e8                                        	mov    %rbp,%r8
  20180c:	4d 89 f9                                        	mov    %r15,%r9
  20180f:	e8 bc 09 00 00                                  	call   2021d0 <emuella_j2k_transform::transform_line_forward_first_low_bounded>
  201814:	48 8b 7c 24 18                                  	mov    0x18(%rsp),%rdi
  201819:	48 89 ee                                        	mov    %rbp,%rsi
  20181c:	4a 8d 14 bd 00 00 00 00                         	lea    0x0(,%r15,4),%rdx
  201824:	ff 15 8e c5 06 00                               	call   *0x6c58e(%rip)        # 26ddb8 <memcpy@GLIBC_2.14>
  20182a:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  20182f:	49 01 dc                                        	add    %rbx,%r12
  201832:	48 8d 04 9d 00 00 00 00                         	lea    0x0(,%rbx,4),%rax
  20183a:	49 01 c5                                        	add    %rax,%r13
  20183d:	49 ff ce                                        	dec    %r14
  201840:	48 8b 7c 24 20                                  	mov    0x20(%rsp),%rdi
  201845:	75 90                                           	jne    2017d7 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x417>
  201847:	c7 07 ff ff ff ff                               	movl   $0xffffffff,(%rdi)
  20184d:	e9 aa fb ff ff                                  	jmp    2013fc <emuella_j2k_transform::forward_reversible_5_3_bounded+0x3c>
  201852:	49 39 d7                                        	cmp    %rdx,%r15
  201855:	0f 87 ae 00 00 00                               	ja     201909 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x549>
  20185b:	48 8d 0d 66 93 06 00                            	lea    0x69366(%rip),%rcx        # 26abc8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3c20>
  201862:	31 ff                                           	xor    %edi,%edi
  201864:	4c 89 fe                                        	mov    %r15,%rsi
  201867:	48 8b 54 24 38                                  	mov    0x38(%rsp),%rdx
  20186c:	ff 15 7e c7 06 00                               	call   *0x6c77e(%rip)        # 26dff0 <_DYNAMIC+0x470>
  201872:	48 8d 3d 63 0f e2 ff                            	lea    -0x1df09d(%rip),%rdi        # 227dc <anon.c3db339937c4b26e029c5c277d8a0513.142.llvm.11782617808337995929+0xb9>
  201879:	48 8d 15 18 93 06 00                            	lea    0x69318(%rip),%rdx        # 26ab98 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3bf0>
  201880:	be 13 00 00 00                                  	mov    $0x13,%esi
  201885:	ff 15 9d c5 06 00                               	call   *0x6c59d(%rip)        # 26de28 <_DYNAMIC+0x2a8>
  20188b:	48 8d 3d 4a 0f e2 ff                            	lea    -0x1df0b6(%rip),%rdi        # 227dc <anon.c3db339937c4b26e029c5c277d8a0513.142.llvm.11782617808337995929+0xb9>
  201892:	48 8d 15 17 93 06 00                            	lea    0x69317(%rip),%rdx        # 26abb0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3c08>
  201899:	be 13 00 00 00                                  	mov    $0x13,%esi
  20189e:	ff 15 84 c5 06 00                               	call   *0x6c584(%rip)        # 26de28 <_DYNAMIC+0x2a8>
  2018a4:	31 ff                                           	xor    %edi,%edi
  2018a6:	31 c9                                           	xor    %ecx,%ecx
  2018a8:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
  2018b0:	48 39 d7                                        	cmp    %rdx,%rdi
  2018b3:	73 42                                           	jae    2018f7 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x537>
  2018b5:	8b 34 b8                                        	mov    (%rax,%rdi,4),%esi
  2018b8:	89 74 0d 00                                     	mov    %esi,0x0(%rbp,%rcx,1)
  2018bc:	48 01 df                                        	add    %rbx,%rdi
  2018bf:	48 83 c1 04                                     	add    $0x4,%rcx
  2018c3:	49 39 cd                                        	cmp    %rcx,%r13
  2018c6:	75 e8                                           	jne    2018b0 <emuella_j2k_transform::forward_reversible_5_3_bounded+0x4f0>
  2018c8:	48 8d 0d 29 93 06 00                            	lea    0x69329(%rip),%rcx        # 26abf8 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3c50>
  2018cf:	31 ff                                           	xor    %edi,%edi
  2018d1:	4c 89 f6                                        	mov    %r14,%rsi
  2018d4:	48 8b 54 24 38                                  	mov    0x38(%rsp),%rdx
  2018d9:	ff 15 11 c7 06 00                               	call   *0x6c711(%rip)        # 26dff0 <_DYNAMIC+0x470>
  2018df:	48 8d 15 82 92 06 00                            	lea    0x69282(%rip),%rdx        # 26ab68 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3bc0>
  2018e6:	4c 89 e7                                        	mov    %r12,%rdi
  2018e9:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  2018ee:	ff 15 b4 c4 06 00                               	call   *0x6c4b4(%rip)        # 26dda8 <_DYNAMIC+0x228>
  2018f4:	48 89 d7                                        	mov    %rdx,%rdi
  2018f7:	48 8d 15 52 92 06 00                            	lea    0x69252(%rip),%rdx        # 26ab50 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3ba8>
  2018fe:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  201903:	ff 15 9f c4 06 00                               	call   *0x6c49f(%rip)        # 26dda8 <_DYNAMIC+0x228>
  201909:	45 31 e4                                        	xor    %r12d,%r12d
  20190c:	4c 89 fe                                        	mov    %r15,%rsi
  20190f:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  201914:	48 8d 0d c5 92 06 00                            	lea    0x692c5(%rip),%rcx        # 26abe0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3c38>
  20191b:	4c 89 e7                                        	mov    %r12,%rdi
  20191e:	ff 15 cc c6 06 00                               	call   *0x6c6cc(%rip)        # 26dff0 <_DYNAMIC+0x470>
