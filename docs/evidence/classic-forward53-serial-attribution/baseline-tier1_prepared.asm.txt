Disassembly of section .text:

00000000001d1320 <emuella_j2k_tier1::encode_prepared_baseline_code_block>:
  1d1320:	55                                              	push   %rbp
  1d1321:	41 57                                           	push   %r15
  1d1323:	41 56                                           	push   %r14
  1d1325:	41 55                                           	push   %r13
  1d1327:	41 54                                           	push   %r12
  1d1329:	53                                              	push   %rbx
  1d132a:	48 81 ec 28 01 00 00                            	sub    $0x128,%rsp
  1d1331:	49 89 ca                                        	mov    %rcx,%r10
  1d1334:	48 89 74 24 10                                  	mov    %rsi,0x10(%rsp)
  1d1339:	48 c1 e9 20                                     	shr    $0x20,%rcx
  1d133d:	0f bd f2                                        	bsr    %edx,%esi
  1d1340:	83 f6 1f                                        	xor    $0x1f,%esi
  1d1343:	b0 20                                           	mov    $0x20,%al
  1d1345:	40 28 f0                                        	sub    %sil,%al
  1d1348:	28 c1                                           	sub    %al,%cl
  1d134a:	73 1b                                           	jae    1d1367 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x47>
  1d134c:	c6 07 03                                        	movb   $0x3,(%rdi)
  1d134f:	48 8d 05 7c 11 e5 ff                            	lea    -0x1aee84(%rip),%rax        # 224d2 <anon.c3db339937c4b26e029c5c277d8a0513.96.llvm.11782617808337995929+0x41>
  1d1356:	48 89 47 08                                     	mov    %rax,0x8(%rdi)
  1d135a:	48 c7 47 10 32 00 00 00                         	movq   $0x32,0x10(%rdi)
  1d1362:	e9 0b 16 00 00                                  	jmp    1d2972 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1652>
  1d1367:	49 c1 ea 28                                     	shr    $0x28,%r10
  1d136b:	49 8b 50 10                                     	mov    0x10(%r8),%rdx
  1d136f:	0f 57 c0                                        	xorps  %xmm0,%xmm0
  1d1372:	0f 11 44 24 56                                  	movups %xmm0,0x56(%rsp)
  1d1377:	0f 11 44 24 46                                  	movups %xmm0,0x46(%rsp)
  1d137c:	40 80 f6 1f                                     	xor    $0x1f,%sil
  1d1380:	40 0f b6 c6                                     	movzbl %sil,%eax
  1d1384:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d1387:	fe c0                                           	inc    %al
  1d1389:	0f b6 c0                                        	movzbl %al,%eax
  1d138c:	89 44 24 1c                                     	mov    %eax,0x1c(%rsp)
  1d1390:	4c 89 44 24 30                                  	mov    %r8,0x30(%rsp)
  1d1395:	48 89 54 24 28                                  	mov    %rdx,0x28(%rsp)
  1d139a:	66 c7 44 24 44 04 00                            	movw   $0x4,0x44(%rsp)
  1d13a1:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d13aa:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d13b2:	48 b8 03 00 2e 00 00 08 08 00                   	movabs $0x80800002e0003,%rax
  1d13bc:	48 89 44 24 66                                  	mov    %rax,0x66(%rsp)
  1d13c1:	41 0f b6 c2                                     	movzbl %r10b,%eax
  1d13c5:	89 44 24 70                                     	mov    %eax,0x70(%rsp)
  1d13c9:	41 f6 c2 02                                     	test   $0x2,%r10b
  1d13cd:	4c 89 8c 24 e0 00 00 00                         	mov    %r9,0xe0(%rsp)
  1d13d5:	4c 89 54 24 08                                  	mov    %r10,0x8(%rsp)
  1d13da:	89 74 24 74                                     	mov    %esi,0x74(%rsp)
  1d13de:	48 89 bc 24 08 01 00 00                         	mov    %rdi,0x108(%rsp)
  1d13e6:	48 89 8c 24 20 01 00 00                         	mov    %rcx,0x120(%rsp)
  1d13ee:	48 89 94 24 18 01 00 00                         	mov    %rdx,0x118(%rsp)
  1d13f6:	0f 85 bf 00 00 00                               	jne    1d14bb <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x19b>
  1d13fc:	44 89 d0                                        	mov    %r10d,%eax
  1d13ff:	c0 e8 04                                        	shr    $0x4,%al
  1d1402:	24 01                                           	and    $0x1,%al
  1d1404:	88 44 24 20                                     	mov    %al,0x20(%rsp)
  1d1408:	41 f6 c2 08                                     	test   $0x8,%r10b
  1d140c:	0f 85 47 0a 00 00                               	jne    1d1e59 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xb39>
  1d1412:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
  1d1417:	4c 8b 60 40                                     	mov    0x40(%rax),%r12
  1d141b:	48 8b 78 38                                     	mov    0x38(%rax),%rdi
  1d141f:	48 8b 08                                        	mov    (%rax),%rcx
  1d1422:	48 89 4c 24 78                                  	mov    %rcx,0x78(%rsp)
  1d1427:	4c 8b 70 08                                     	mov    0x8(%rax),%r14
  1d142b:	4c 8b 40 30                                     	mov    0x30(%rax),%r8
  1d142f:	4c 89 84 24 d0 00 00 00                         	mov    %r8,0xd0(%rsp)
  1d1437:	4b 8d 04 40                                     	lea    (%r8,%r8,2),%rax
  1d143b:	48 83 c0 fd                                     	add    $0xfffffffffffffffd,%rax
  1d143f:	48 b9 ab aa aa aa aa aa aa aa                   	movabs $0xaaaaaaaaaaaaaaab,%rcx
  1d1449:	48 89 84 24 b0 00 00 00                         	mov    %rax,0xb0(%rsp)
  1d1451:	48 f7 e1                                        	mul    %rcx
  1d1454:	4b 8d 04 40                                     	lea    (%r8,%r8,2),%rax
  1d1458:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
  1d1460:	48 d1 ea                                        	shr    $1,%rdx
  1d1463:	8d 42 01                                        	lea    0x1(%rdx),%eax
  1d1466:	83 e0 07                                        	and    $0x7,%eax
  1d1469:	83 e2 07                                        	and    $0x7,%edx
  1d146c:	48 89 94 24 a8 00 00 00                         	mov    %rdx,0xa8(%rsp)
  1d1474:	48 f7 d8                                        	neg    %rax
  1d1477:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
  1d147f:	48 89 bc 24 80 00 00 00                         	mov    %rdi,0x80(%rsp)
  1d1487:	48 8d 47 01                                     	lea    0x1(%rdi),%rax
  1d148b:	48 89 84 24 10 01 00 00                         	mov    %rax,0x110(%rsp)
  1d1493:	45 31 ff                                        	xor    %r15d,%r15d
  1d1496:	48 8d 84 24 e8 00 00 00                         	lea    0xe8(%rsp),%rax
  1d149e:	48 89 84 24 d8 00 00 00                         	mov    %rax,0xd8(%rsp)
  1d14a6:	4c 89 b4 24 90 00 00 00                         	mov    %r14,0x90(%rsp)
  1d14ae:	4c 89 a4 24 88 00 00 00                         	mov    %r12,0x88(%rsp)
  1d14b6:	e9 e5 00 00 00                                  	jmp    1d15a0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x280>
  1d14bb:	41 f6 c2 08                                     	test   $0x8,%r10b
  1d14bf:	0f 85 84 0e 00 00                               	jne    1d2349 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1029>
  1d14c5:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
  1d14ca:	48 8b 48 40                                     	mov    0x40(%rax),%rcx
  1d14ce:	48 89 8c 24 90 00 00 00                         	mov    %rcx,0x90(%rsp)
  1d14d6:	48 8b 78 38                                     	mov    0x38(%rax),%rdi
  1d14da:	48 89 bc 24 88 00 00 00                         	mov    %rdi,0x88(%rsp)
  1d14e2:	48 8b 08                                        	mov    (%rax),%rcx
  1d14e5:	48 89 8c 24 80 00 00 00                         	mov    %rcx,0x80(%rsp)
  1d14ed:	4c 8b 78 08                                     	mov    0x8(%rax),%r15
  1d14f1:	48 8b 40 30                                     	mov    0x30(%rax),%rax
  1d14f5:	48 89 44 24 78                                  	mov    %rax,0x78(%rsp)
  1d14fa:	48 8d 0c 40                                     	lea    (%rax,%rax,2),%rcx
  1d14fe:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  1d1503:	45 89 d4                                        	mov    %r10d,%r12d
  1d1506:	41 c0 ec 04                                     	shr    $0x4,%r12b
  1d150a:	41 80 e4 01                                     	and    $0x1,%r12b
  1d150e:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
  1d1512:	48 83 c0 fd                                     	add    $0xfffffffffffffffd,%rax
  1d1516:	48 b9 ab aa aa aa aa aa aa aa                   	movabs $0xaaaaaaaaaaaaaaab,%rcx
  1d1520:	48 89 84 24 b0 00 00 00                         	mov    %rax,0xb0(%rsp)
  1d1528:	48 f7 e1                                        	mul    %rcx
  1d152b:	48 d1 ea                                        	shr    $1,%rdx
  1d152e:	8d 42 01                                        	lea    0x1(%rdx),%eax
  1d1531:	83 e0 07                                        	and    $0x7,%eax
  1d1534:	83 e2 07                                        	and    $0x7,%edx
  1d1537:	48 89 94 24 a8 00 00 00                         	mov    %rdx,0xa8(%rsp)
  1d153f:	48 f7 d8                                        	neg    %rax
  1d1542:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
  1d154a:	48 8d 47 01                                     	lea    0x1(%rdi),%rax
  1d154e:	48 89 84 24 d0 00 00 00                         	mov    %rax,0xd0(%rsp)
  1d1556:	45 31 f6                                        	xor    %r14d,%r14d
  1d1559:	48 8d 84 24 f0 00 00 00                         	lea    0xf0(%rsp),%rax
  1d1561:	48 89 84 24 d8 00 00 00                         	mov    %rax,0xd8(%rsp)
  1d1569:	4c 89 bc 24 98 00 00 00                         	mov    %r15,0x98(%rsp)
  1d1571:	e9 9a 04 00 00                                  	jmp    1d1a10 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6f0>
  1d1576:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d157f:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d1587:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d1590:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d1596:	8b 74 24 74                                     	mov    0x74(%rsp),%esi
  1d159a:	0f 84 da 12 00 00                               	je     1d287a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x155a>
  1d15a0:	41 0f b7 c7                                     	movzwl %r15w,%eax
  1d15a4:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d15aa:	c1 e8 11                                        	shr    $0x11,%eax
  1d15ad:	8d 0c 40                                        	lea    (%rax,%rax,2),%ecx
  1d15b0:	31 d2                                           	xor    %edx,%edx
  1d15b2:	44 89 fb                                        	mov    %r15d,%ebx
  1d15b5:	66 29 cb                                        	sub    %cx,%bx
  1d15b8:	0f 95 c2                                        	setne  %dl
  1d15bb:	01 c2                                           	add    %eax,%edx
  1d15bd:	b0 03                                           	mov    $0x3,%al
  1d15bf:	89 f1                                           	mov    %esi,%ecx
  1d15c1:	28 d1                                           	sub    %dl,%cl
  1d15c3:	0f 82 a0 12 00 00                               	jb     1d2869 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1549>
  1d15c9:	44 89 fd                                        	mov    %r15d,%ebp
  1d15cc:	66 85 db                                        	test   %bx,%bx
  1d15cf:	0f 95 c2                                        	setne  %dl
  1d15d2:	66 41 83 ff 0a                                  	cmp    $0xa,%r15w
  1d15d7:	41 0f 93 c5                                     	setae  %r13b
  1d15db:	45 20 d5                                        	and    %r10b,%r13b
  1d15de:	41 20 d5                                        	and    %dl,%r13b
  1d15e1:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  1d15e6:	88 4a 48                                        	mov    %cl,0x48(%rdx)
  1d15e9:	66 85 db                                        	test   %bx,%bx
  1d15ec:	74 32                                           	je     1d1620 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x300>
  1d15ee:	0f b7 c3                                        	movzwl %bx,%eax
  1d15f1:	83 f8 01                                        	cmp    $0x1,%eax
  1d15f4:	0f 85 86 01 00 00                               	jne    1d1780 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x460>
  1d15fa:	45 84 ed                                        	test   %r13b,%r13b
  1d15fd:	0f 84 93 01 00 00                               	je     1d1796 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x476>
  1d1603:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d1608:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d160d:	e8 6e 8c fe ff                                  	call   1ba280 <emuella_j2k_tier1::significance_propagation_pass_encode_raw::<false>>
  1d1612:	e9 9f 01 00 00                                  	jmp    1d17b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x496>
  1d1617:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d1620:	45 84 ed                                        	test   %r13b,%r13b
  1d1623:	4c 8b bc 24 80 00 00 00                         	mov    0x80(%rsp),%r15
  1d162b:	0f 85 85 12 00 00                               	jne    1d28b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1596>
  1d1631:	48 8d bc 24 b8 00 00 00                         	lea    0xb8(%rsp),%rdi
  1d1639:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1d163e:	48 8d 54 24 28                                  	lea    0x28(%rsp),%rdx
  1d1643:	e8 d8 e2 fd ff                                  	call   1af920 <emuella_j2k_tier1::cleanup_pass_encode::<false>>
  1d1648:	0f b6 84 24 b8 00 00 00                         	movzbl 0xb8(%rsp),%eax
  1d1650:	3c ff                                           	cmp    $0xff,%al
  1d1652:	0f 85 6f 12 00 00                               	jne    1d28c7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x15a7>
  1d1658:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d165d:	41 f6 c2 20                                     	test   $0x20,%r10b
  1d1661:	74 4f                                           	je     1d16b2 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x392>
  1d1663:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1668:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d166d:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d1672:	e8 89 85 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d1677:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d167c:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d1681:	31 d2                                           	xor    %edx,%edx
  1d1683:	e8 78 85 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d1688:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d168d:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d1692:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d1697:	e8 64 85 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d169c:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d16a1:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d16a6:	31 d2                                           	xor    %edx,%edx
  1d16a8:	e8 53 85 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d16ad:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d16b2:	4d 85 e4                                        	test   %r12,%r12
  1d16b5:	44 0f b6 44 24 20                               	movzbl 0x20(%rsp),%r8d
  1d16bb:	4c 8b 4c 24 78                                  	mov    0x78(%rsp),%r9
  1d16c0:	4c 8b 9c 24 d0 00 00 00                         	mov    0xd0(%rsp),%r11
  1d16c8:	0f 84 f3 00 00 00                               	je     1d17c1 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4a1>
  1d16ce:	4d 85 db                                        	test   %r11,%r11
  1d16d1:	0f 84 d1 02 00 00                               	je     1d19a8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x688>
  1d16d7:	31 c0                                           	xor    %eax,%eax
  1d16d9:	eb 0e                                           	jmp    1d16e9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x3c9>
  1d16db:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
  1d16e0:	4c 39 e0                                        	cmp    %r12,%rax
  1d16e3:	0f 84 d8 00 00 00                               	je     1d17c1 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4a1>
  1d16e9:	48 ff c0                                        	inc    %rax
  1d16ec:	48 89 c7                                        	mov    %rax,%rdi
  1d16ef:	49 0f af ff                                     	imul   %r15,%rdi
  1d16f3:	48 ff c7                                        	inc    %rdi
  1d16f6:	48 89 fe                                        	mov    %rdi,%rsi
  1d16f9:	4c 01 de                                        	add    %r11,%rsi
  1d16fc:	0f 82 98 12 00 00                               	jb     1d299a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x167a>
  1d1702:	4c 39 f6                                        	cmp    %r14,%rsi
  1d1705:	0f 87 8f 12 00 00                               	ja     1d299a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x167a>
  1d170b:	48 8d 0c 7f                                     	lea    (%rdi,%rdi,2),%rcx
  1d170f:	4c 01 c9                                        	add    %r9,%rcx
  1d1712:	48 89 ca                                        	mov    %rcx,%rdx
  1d1715:	83 bc 24 a8 00 00 00 07                         	cmpl   $0x7,0xa8(%rsp)
  1d171d:	74 1e                                           	je     1d173d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x41d>
  1d171f:	48 8b b4 24 98 00 00 00                         	mov    0x98(%rsp),%rsi
  1d1727:	48 89 ca                                        	mov    %rcx,%rdx
  1d172a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d1730:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d1734:	48 83 c2 03                                     	add    $0x3,%rdx
  1d1738:	48 ff c6                                        	inc    %rsi
  1d173b:	75 f3                                           	jne    1d1730 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x410>
  1d173d:	48 83 bc 24 b0 00 00 00 15                      	cmpq   $0x15,0xb0(%rsp)
  1d1746:	72 98                                           	jb     1d16e0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x3c0>
  1d1748:	48 03 8c 24 a0 00 00 00                         	add    0xa0(%rsp),%rcx
  1d1750:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d1754:	c6 42 04 00                                     	movb   $0x0,0x4(%rdx)
  1d1758:	c6 42 07 00                                     	movb   $0x0,0x7(%rdx)
  1d175c:	c6 42 0a 00                                     	movb   $0x0,0xa(%rdx)
  1d1760:	c6 42 0d 00                                     	movb   $0x0,0xd(%rdx)
  1d1764:	c6 42 10 00                                     	movb   $0x0,0x10(%rdx)
  1d1768:	c6 42 13 00                                     	movb   $0x0,0x13(%rdx)
  1d176c:	c6 42 16 00                                     	movb   $0x0,0x16(%rdx)
  1d1770:	48 83 c2 18                                     	add    $0x18,%rdx
  1d1774:	48 39 ca                                        	cmp    %rcx,%rdx
  1d1777:	75 d7                                           	jne    1d1750 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x430>
  1d1779:	e9 62 ff ff ff                                  	jmp    1d16e0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x3c0>
  1d177e:	66 90                                           	xchg   %ax,%ax
  1d1780:	45 84 ed                                        	test   %r13b,%r13b
  1d1783:	74 22                                           	je     1d17a7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x487>
  1d1785:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d178a:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d178f:	e8 2c 12 00 00                                  	call   1d29c0 <emuella_j2k_tier1::magnitude_refinement_pass_encode_raw>
  1d1794:	eb 20                                           	jmp    1d17b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x496>
  1d1796:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d179b:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d17a0:	e8 fb 73 fe ff                                  	call   1b8ba0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>>
  1d17a5:	eb 0f                                           	jmp    1d17b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x496>
  1d17a7:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d17ac:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d17b1:	e8 6a 53 fe ff                                  	call   1b6b20 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>>
  1d17b6:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d17bb:	44 0f b6 44 24 20                               	movzbl 0x20(%rsp),%r8d
  1d17c1:	44 8d 7d 01                                     	lea    0x1(%rbp),%r15d
  1d17c5:	41 f6 c2 04                                     	test   $0x4,%r10b
  1d17c9:	75 35                                           	jne    1d1800 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4e0>
  1d17cb:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d17d1:	74 2d                                           	je     1d1800 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4e0>
  1d17d3:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d17d7:	0f 84 b3 fd ff ff                               	je     1d1590 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x270>
  1d17dd:	66 83 fd 09                                     	cmp    $0x9,%bp
  1d17e1:	74 1d                                           	je     1d1800 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4e0>
  1d17e3:	0f 96 c0                                        	setbe  %al
  1d17e6:	66 83 fb 01                                     	cmp    $0x1,%bx
  1d17ea:	0f 94 c1                                        	sete   %cl
  1d17ed:	08 c1                                           	or     %al,%cl
  1d17ef:	0f 85 9b fd ff ff                               	jne    1d1590 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x270>
  1d17f5:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  1d1800:	45 84 ed                                        	test   %r13b,%r13b
  1d1803:	74 3b                                           	je     1d1840 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x520>
  1d1805:	0f b6 44 24 6c                                  	movzbl 0x6c(%rsp),%eax
  1d180a:	0f b6 54 24 6b                                  	movzbl 0x6b(%rsp),%edx
  1d180f:	48 8b 4c 24 28                                  	mov    0x28(%rsp),%rcx
  1d1814:	48 8b 5c 24 30                                  	mov    0x30(%rsp),%rbx
  1d1819:	4c 8b 6b 10                                     	mov    0x10(%rbx),%r13
  1d181d:	49 39 cd                                        	cmp    %rcx,%r13
  1d1820:	76 36                                           	jbe    1d1858 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x538>
  1d1822:	38 d0                                           	cmp    %dl,%al
  1d1824:	72 3a                                           	jb     1d1860 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x540>
  1d1826:	48 8b 53 08                                     	mov    0x8(%rbx),%rdx
  1d182a:	48 01 ca                                        	add    %rcx,%rdx
  1d182d:	4c 01 ea                                        	add    %r13,%rdx
  1d1830:	48 f7 d1                                        	not    %rcx
  1d1833:	80 3c 11 ff                                     	cmpb   $0xff,(%rcx,%rdx,1)
  1d1837:	0f 85 c3 00 00 00                               	jne    1d1900 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5e0>
  1d183d:	eb 21                                           	jmp    1d1860 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x540>
  1d183f:	90                                              	nop
  1d1840:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1845:	8b 74 24 70                                     	mov    0x70(%rsp),%esi
  1d1849:	e8 f2 ef ff ff                                  	call   1d0840 <emuella_j2k_tier1::terminate_arithmetic_segment>
  1d184e:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d1853:	e9 a8 00 00 00                                  	jmp    1d1900 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5e0>
  1d1858:	38 d0                                           	cmp    %dl,%al
  1d185a:	0f 83 a0 00 00 00                               	jae    1d1900 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5e0>
  1d1860:	44 0f b6 74 24 6a                               	movzbl 0x6a(%rsp),%r14d
  1d1866:	84 c0                                           	test   %al,%al
  1d1868:	74 5c                                           	je     1d18c6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5a6>
  1d186a:	89 c2                                           	mov    %eax,%edx
  1d186c:	80 e2 03                                        	and    $0x3,%dl
  1d186f:	3c 04                                           	cmp    $0x4,%al
  1d1871:	72 36                                           	jb     1d18a9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x589>
  1d1873:	89 c6                                           	mov    %eax,%esi
  1d1875:	40 80 e6 fc                                     	and    $0xfc,%sil
  1d1879:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d1880:	8d 48 06                                        	lea    0x6(%rax),%ecx
  1d1883:	80 e1 07                                        	and    $0x7,%cl
  1d1886:	44 89 c7                                        	mov    %r8d,%edi
  1d1889:	40 d2 e7                                        	shl    %cl,%dil
  1d188c:	44 08 f7                                        	or     %r14b,%dil
  1d188f:	04 fc                                           	add    $0xfc,%al
  1d1891:	89 c1                                           	mov    %eax,%ecx
  1d1893:	80 e1 07                                        	and    $0x7,%cl
  1d1896:	45 89 c6                                        	mov    %r8d,%r14d
  1d1899:	41 d2 e6                                        	shl    %cl,%r14b
  1d189c:	41 08 fe                                        	or     %dil,%r14b
  1d189f:	40 80 c6 fc                                     	add    $0xfc,%sil
  1d18a3:	75 db                                           	jne    1d1880 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x560>
  1d18a5:	84 d2                                           	test   %dl,%dl
  1d18a7:	74 1d                                           	je     1d18c6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5a6>
  1d18a9:	fe c8                                           	dec    %al
  1d18ab:	31 f6                                           	xor    %esi,%esi
  1d18ad:	0f 1f 00                                        	nopl   (%rax)
  1d18b0:	89 c1                                           	mov    %eax,%ecx
  1d18b2:	80 e1 07                                        	and    $0x7,%cl
  1d18b5:	89 f7                                           	mov    %esi,%edi
  1d18b7:	40 d2 e7                                        	shl    %cl,%dil
  1d18ba:	41 08 fe                                        	or     %dil,%r14b
  1d18bd:	44 30 c6                                        	xor    %r8b,%sil
  1d18c0:	fe c8                                           	dec    %al
  1d18c2:	fe ca                                           	dec    %dl
  1d18c4:	75 ea                                           	jne    1d18b0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x590>
  1d18c6:	4c 3b 2b                                        	cmp    (%rbx),%r13
  1d18c9:	75 0e                                           	jne    1d18d9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x5b9>
  1d18cb:	48 89 df                                        	mov    %rbx,%rdi
  1d18ce:	ff 15 1c c5 09 00                               	call   *0x9c51c(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1d18d4:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d18d9:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1d18dd:	46 88 34 28                                     	mov    %r14b,(%rax,%r13,1)
  1d18e1:	49 ff c5                                        	inc    %r13
  1d18e4:	4c 89 6b 10                                     	mov    %r13,0x10(%rbx)
  1d18e8:	41 80 fe ff                                     	cmp    $0xff,%r14b
  1d18ec:	0f 94 c0                                        	sete   %al
  1d18ef:	b1 08                                           	mov    $0x8,%cl
  1d18f1:	28 c1                                           	sub    %al,%cl
  1d18f3:	88 4c 24 6b                                     	mov    %cl,0x6b(%rsp)
  1d18f7:	88 4c 24 6c                                     	mov    %cl,0x6c(%rsp)
  1d18fb:	c6 44 24 6a 00                                  	movb   $0x0,0x6a(%rsp)
  1d1900:	4c 8b a4 24 e0 00 00 00                         	mov    0xe0(%rsp),%r12
  1d1908:	4d 85 e4                                        	test   %r12,%r12
  1d190b:	74 2e                                           	je     1d193b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x61b>
  1d190d:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d1912:	48 8b 58 10                                     	mov    0x10(%rax),%rbx
  1d1916:	48 2b 5c 24 28                                  	sub    0x28(%rsp),%rbx
  1d191b:	4d 8b 74 24 10                                  	mov    0x10(%r12),%r14
  1d1920:	4d 3b 34 24                                     	cmp    (%r12),%r14
  1d1924:	0f 84 ac 00 00 00                               	je     1d19d6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6b6>
  1d192a:	49 8b 44 24 08                                  	mov    0x8(%r12),%rax
  1d192f:	4a 89 1c f0                                     	mov    %rbx,(%rax,%r14,8)
  1d1933:	49 ff c6                                        	inc    %r14
  1d1936:	4d 89 74 24 10                                  	mov    %r14,0x10(%r12)
  1d193b:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d1941:	4c 8b b4 24 90 00 00 00                         	mov    0x90(%rsp),%r14
  1d1949:	4c 8b a4 24 88 00 00 00                         	mov    0x88(%rsp),%r12
  1d1951:	0f 83 39 fc ff ff                               	jae    1d1590 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x270>
  1d1957:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d195c:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d1960:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1d1965:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d1969:	0f 84 07 fc ff ff                               	je     1d1576 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x256>
  1d196f:	66 83 fd 09                                     	cmp    $0x9,%bp
  1d1973:	0f 82 fd fb ff ff                               	jb     1d1576 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x256>
  1d1979:	41 0f b7 c7                                     	movzwl %r15w,%eax
  1d197d:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d1983:	c1 e8 11                                        	shr    $0x11,%eax
  1d1986:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d1989:	44 89 f9                                        	mov    %r15d,%ecx
  1d198c:	29 c1                                           	sub    %eax,%ecx
  1d198e:	66 85 c9                                        	test   %cx,%cx
  1d1991:	0f 84 df fb ff ff                               	je     1d1576 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x256>
  1d1997:	66 c7 44 24 6a 00 08                            	movw   $0x800,0x6a(%rsp)
  1d199e:	c6 44 24 6c 08                                  	movb   $0x8,0x6c(%rsp)
  1d19a3:	e9 e8 fb ff ff                                  	jmp    1d1590 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x270>
  1d19a8:	48 8b bc 24 10 01 00 00                         	mov    0x110(%rsp),%rdi
  1d19b0:	4c 89 e0                                        	mov    %r12,%rax
  1d19b3:	66 66 66 66 2e 0f 1f 84 00 00 00 00 00          	data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
  1d19c0:	4c 39 f7                                        	cmp    %r14,%rdi
  1d19c3:	0f 87 ce 0f 00 00                               	ja     1d2997 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1677>
  1d19c9:	4c 01 ff                                        	add    %r15,%rdi
  1d19cc:	48 ff c8                                        	dec    %rax
  1d19cf:	75 ef                                           	jne    1d19c0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6a0>
  1d19d1:	e9 eb fd ff ff                                  	jmp    1d17c1 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x4a1>
  1d19d6:	4c 89 e7                                        	mov    %r12,%rdi
  1d19d9:	ff 15 b1 cc 09 00                               	call   *0x9ccb1(%rip)        # 26e690 <_DYNAMIC+0xb10>
  1d19df:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d19e4:	e9 41 ff ff ff                                  	jmp    1d192a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x60a>
  1d19e9:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d19f2:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d19fa:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d1a00:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d1a06:	8b 74 24 74                                     	mov    0x74(%rsp),%esi
  1d1a0a:	0f 84 6a 0e 00 00                               	je     1d287a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x155a>
  1d1a10:	41 0f b7 c6                                     	movzwl %r14w,%eax
  1d1a14:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d1a1a:	c1 e8 11                                        	shr    $0x11,%eax
  1d1a1d:	8d 0c 40                                        	lea    (%rax,%rax,2),%ecx
  1d1a20:	31 d2                                           	xor    %edx,%edx
  1d1a22:	44 89 f5                                        	mov    %r14d,%ebp
  1d1a25:	66 29 cd                                        	sub    %cx,%bp
  1d1a28:	0f 95 c2                                        	setne  %dl
  1d1a2b:	01 c2                                           	add    %eax,%edx
  1d1a2d:	b0 03                                           	mov    $0x3,%al
  1d1a2f:	89 f1                                           	mov    %esi,%ecx
  1d1a31:	28 d1                                           	sub    %dl,%cl
  1d1a33:	0f 82 30 0e 00 00                               	jb     1d2869 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1549>
  1d1a39:	44 89 f3                                        	mov    %r14d,%ebx
  1d1a3c:	66 85 ed                                        	test   %bp,%bp
  1d1a3f:	0f 95 c2                                        	setne  %dl
  1d1a42:	66 41 83 fe 0a                                  	cmp    $0xa,%r14w
  1d1a47:	41 0f 93 c5                                     	setae  %r13b
  1d1a4b:	45 20 d5                                        	and    %r10b,%r13b
  1d1a4e:	41 20 d5                                        	and    %dl,%r13b
  1d1a51:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  1d1a56:	88 4a 48                                        	mov    %cl,0x48(%rdx)
  1d1a59:	66 85 ed                                        	test   %bp,%bp
  1d1a5c:	74 32                                           	je     1d1a90 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x770>
  1d1a5e:	0f b7 c5                                        	movzwl %bp,%eax
  1d1a61:	83 f8 01                                        	cmp    $0x1,%eax
  1d1a64:	0f 85 86 01 00 00                               	jne    1d1bf0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x8d0>
  1d1a6a:	45 84 ed                                        	test   %r13b,%r13b
  1d1a6d:	0f 84 93 01 00 00                               	je     1d1c06 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x8e6>
  1d1a73:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d1a78:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d1a7d:	e8 fe 87 fe ff                                  	call   1ba280 <emuella_j2k_tier1::significance_propagation_pass_encode_raw::<false>>
  1d1a82:	e9 9f 01 00 00                                  	jmp    1d1c26 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x906>
  1d1a87:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d1a90:	45 84 ed                                        	test   %r13b,%r13b
  1d1a93:	4c 8b b4 24 90 00 00 00                         	mov    0x90(%rsp),%r14
  1d1a9b:	0f 85 15 0e 00 00                               	jne    1d28b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1596>
  1d1aa1:	48 8d bc 24 b8 00 00 00                         	lea    0xb8(%rsp),%rdi
  1d1aa9:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1d1aae:	48 8d 54 24 28                                  	lea    0x28(%rsp),%rdx
  1d1ab3:	e8 68 de fd ff                                  	call   1af920 <emuella_j2k_tier1::cleanup_pass_encode::<false>>
  1d1ab8:	0f b6 84 24 b8 00 00 00                         	movzbl 0xb8(%rsp),%eax
  1d1ac0:	3c ff                                           	cmp    $0xff,%al
  1d1ac2:	0f 85 1d 0e 00 00                               	jne    1d28e5 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x15c5>
  1d1ac8:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d1acd:	41 f6 c2 20                                     	test   $0x20,%r10b
  1d1ad1:	74 4f                                           	je     1d1b22 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x802>
  1d1ad3:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1ad8:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d1add:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d1ae2:	e8 19 81 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d1ae7:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1aec:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d1af1:	31 d2                                           	xor    %edx,%edx
  1d1af3:	e8 08 81 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d1af8:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1afd:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d1b02:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d1b07:	e8 f4 80 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d1b0c:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1b11:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d1b16:	31 d2                                           	xor    %edx,%edx
  1d1b18:	e8 e3 80 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d1b1d:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d1b22:	4d 85 f6                                        	test   %r14,%r14
  1d1b25:	4c 8b 84 24 88 00 00 00                         	mov    0x88(%rsp),%r8
  1d1b2d:	4c 8b 8c 24 80 00 00 00                         	mov    0x80(%rsp),%r9
  1d1b35:	4c 8b 5c 24 78                                  	mov    0x78(%rsp),%r11
  1d1b3a:	0f 84 eb 00 00 00                               	je     1d1c2b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x90b>
  1d1b40:	4d 85 db                                        	test   %r11,%r11
  1d1b43:	0f 84 d5 02 00 00                               	je     1d1e1e <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xafe>
  1d1b49:	31 c0                                           	xor    %eax,%eax
  1d1b4b:	eb 0c                                           	jmp    1d1b59 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x839>
  1d1b4d:	0f 1f 00                                        	nopl   (%rax)
  1d1b50:	4c 39 f0                                        	cmp    %r14,%rax
  1d1b53:	0f 84 d2 00 00 00                               	je     1d1c2b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x90b>
  1d1b59:	48 ff c0                                        	inc    %rax
  1d1b5c:	48 89 c7                                        	mov    %rax,%rdi
  1d1b5f:	49 0f af f8                                     	imul   %r8,%rdi
  1d1b63:	48 ff c7                                        	inc    %rdi
  1d1b66:	48 89 fe                                        	mov    %rdi,%rsi
  1d1b69:	4c 01 de                                        	add    %r11,%rsi
  1d1b6c:	0f 82 15 0e 00 00                               	jb     1d2987 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1667>
  1d1b72:	4c 39 fe                                        	cmp    %r15,%rsi
  1d1b75:	0f 87 0c 0e 00 00                               	ja     1d2987 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1667>
  1d1b7b:	48 8d 0c 7f                                     	lea    (%rdi,%rdi,2),%rcx
  1d1b7f:	4c 01 c9                                        	add    %r9,%rcx
  1d1b82:	48 89 ca                                        	mov    %rcx,%rdx
  1d1b85:	83 bc 24 a8 00 00 00 07                         	cmpl   $0x7,0xa8(%rsp)
  1d1b8d:	74 1e                                           	je     1d1bad <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x88d>
  1d1b8f:	48 8b b4 24 a0 00 00 00                         	mov    0xa0(%rsp),%rsi
  1d1b97:	48 89 ca                                        	mov    %rcx,%rdx
  1d1b9a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d1ba0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d1ba4:	48 83 c2 03                                     	add    $0x3,%rdx
  1d1ba8:	48 ff c6                                        	inc    %rsi
  1d1bab:	75 f3                                           	jne    1d1ba0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x880>
  1d1bad:	48 83 bc 24 b0 00 00 00 15                      	cmpq   $0x15,0xb0(%rsp)
  1d1bb6:	72 98                                           	jb     1d1b50 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x830>
  1d1bb8:	48 03 4c 24 20                                  	add    0x20(%rsp),%rcx
  1d1bbd:	0f 1f 00                                        	nopl   (%rax)
  1d1bc0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d1bc4:	c6 42 04 00                                     	movb   $0x0,0x4(%rdx)
  1d1bc8:	c6 42 07 00                                     	movb   $0x0,0x7(%rdx)
  1d1bcc:	c6 42 0a 00                                     	movb   $0x0,0xa(%rdx)
  1d1bd0:	c6 42 0d 00                                     	movb   $0x0,0xd(%rdx)
  1d1bd4:	c6 42 10 00                                     	movb   $0x0,0x10(%rdx)
  1d1bd8:	c6 42 13 00                                     	movb   $0x0,0x13(%rdx)
  1d1bdc:	c6 42 16 00                                     	movb   $0x0,0x16(%rdx)
  1d1be0:	48 83 c2 18                                     	add    $0x18,%rdx
  1d1be4:	48 39 ca                                        	cmp    %rcx,%rdx
  1d1be7:	75 d7                                           	jne    1d1bc0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x8a0>
  1d1be9:	e9 62 ff ff ff                                  	jmp    1d1b50 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x830>
  1d1bee:	66 90                                           	xchg   %ax,%ax
  1d1bf0:	45 84 ed                                        	test   %r13b,%r13b
  1d1bf3:	74 22                                           	je     1d1c17 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x8f7>
  1d1bf5:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d1bfa:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d1bff:	e8 bc 0d 00 00                                  	call   1d29c0 <emuella_j2k_tier1::magnitude_refinement_pass_encode_raw>
  1d1c04:	eb 20                                           	jmp    1d1c26 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x906>
  1d1c06:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d1c0b:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d1c10:	e8 8b 6f fe ff                                  	call   1b8ba0 <emuella_j2k_tier1::significance_propagation_pass_encode::<false>>
  1d1c15:	eb 0f                                           	jmp    1d1c26 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x906>
  1d1c17:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d1c1c:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d1c21:	e8 fa 4e fe ff                                  	call   1b6b20 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<false>>
  1d1c26:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d1c2b:	44 8d 73 01                                     	lea    0x1(%rbx),%r14d
  1d1c2f:	c6 44 24 44 04                                  	movb   $0x4,0x44(%rsp)
  1d1c34:	48 8d 44 24 45                                  	lea    0x45(%rsp),%rax
  1d1c39:	0f 57 c0                                        	xorps  %xmm0,%xmm0
  1d1c3c:	0f 11 40 10                                     	movups %xmm0,0x10(%rax)
  1d1c40:	0f 11 00                                        	movups %xmm0,(%rax)
  1d1c43:	c6 40 20 00                                     	movb   $0x0,0x20(%rax)
  1d1c47:	c7 44 24 66 03 00 2e 00                         	movl   $0x2e0003,0x66(%rsp)
  1d1c4f:	41 f6 c2 04                                     	test   $0x4,%r10b
  1d1c53:	75 2b                                           	jne    1d1c80 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x960>
  1d1c55:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d1c5b:	74 23                                           	je     1d1c80 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x960>
  1d1c5d:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d1c61:	0f 84 99 fd ff ff                               	je     1d1a00 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6e0>
  1d1c67:	66 83 fb 09                                     	cmp    $0x9,%bx
  1d1c6b:	74 13                                           	je     1d1c80 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x960>
  1d1c6d:	0f 96 c0                                        	setbe  %al
  1d1c70:	66 83 fd 01                                     	cmp    $0x1,%bp
  1d1c74:	0f 94 c1                                        	sete   %cl
  1d1c77:	08 c1                                           	or     %al,%cl
  1d1c79:	0f 85 81 fd ff ff                               	jne    1d1a00 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6e0>
  1d1c7f:	90                                              	nop
  1d1c80:	45 84 ed                                        	test   %r13b,%r13b
  1d1c83:	74 3b                                           	je     1d1cc0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9a0>
  1d1c85:	0f b6 44 24 6c                                  	movzbl 0x6c(%rsp),%eax
  1d1c8a:	0f b6 54 24 6b                                  	movzbl 0x6b(%rsp),%edx
  1d1c8f:	48 8b 4c 24 28                                  	mov    0x28(%rsp),%rcx
  1d1c94:	48 8b 6c 24 30                                  	mov    0x30(%rsp),%rbp
  1d1c99:	4c 8b 6d 10                                     	mov    0x10(%rbp),%r13
  1d1c9d:	49 39 cd                                        	cmp    %rcx,%r13
  1d1ca0:	76 36                                           	jbe    1d1cd8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9b8>
  1d1ca2:	38 d0                                           	cmp    %dl,%al
  1d1ca4:	72 3a                                           	jb     1d1ce0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9c0>
  1d1ca6:	48 8b 55 08                                     	mov    0x8(%rbp),%rdx
  1d1caa:	48 01 ca                                        	add    %rcx,%rdx
  1d1cad:	4c 01 ea                                        	add    %r13,%rdx
  1d1cb0:	48 f7 d1                                        	not    %rcx
  1d1cb3:	80 3c 11 ff                                     	cmpb   $0xff,(%rcx,%rdx,1)
  1d1cb7:	0f 85 c4 00 00 00                               	jne    1d1d81 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa61>
  1d1cbd:	eb 21                                           	jmp    1d1ce0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9c0>
  1d1cbf:	90                                              	nop
  1d1cc0:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1cc5:	8b 74 24 70                                     	mov    0x70(%rsp),%esi
  1d1cc9:	e8 72 eb ff ff                                  	call   1d0840 <emuella_j2k_tier1::terminate_arithmetic_segment>
  1d1cce:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d1cd3:	e9 a9 00 00 00                                  	jmp    1d1d81 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa61>
  1d1cd8:	38 d0                                           	cmp    %dl,%al
  1d1cda:	0f 83 a1 00 00 00                               	jae    1d1d81 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa61>
  1d1ce0:	44 0f b6 7c 24 6a                               	movzbl 0x6a(%rsp),%r15d
  1d1ce6:	84 c0                                           	test   %al,%al
  1d1ce8:	74 5c                                           	je     1d1d46 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa26>
  1d1cea:	89 c2                                           	mov    %eax,%edx
  1d1cec:	80 e2 03                                        	and    $0x3,%dl
  1d1cef:	3c 04                                           	cmp    $0x4,%al
  1d1cf1:	72 36                                           	jb     1d1d29 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa09>
  1d1cf3:	89 c6                                           	mov    %eax,%esi
  1d1cf5:	40 80 e6 fc                                     	and    $0xfc,%sil
  1d1cf9:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d1d00:	8d 48 06                                        	lea    0x6(%rax),%ecx
  1d1d03:	80 e1 07                                        	and    $0x7,%cl
  1d1d06:	44 89 e7                                        	mov    %r12d,%edi
  1d1d09:	40 d2 e7                                        	shl    %cl,%dil
  1d1d0c:	44 08 ff                                        	or     %r15b,%dil
  1d1d0f:	04 fc                                           	add    $0xfc,%al
  1d1d11:	89 c1                                           	mov    %eax,%ecx
  1d1d13:	80 e1 07                                        	and    $0x7,%cl
  1d1d16:	45 89 e7                                        	mov    %r12d,%r15d
  1d1d19:	41 d2 e7                                        	shl    %cl,%r15b
  1d1d1c:	41 08 ff                                        	or     %dil,%r15b
  1d1d1f:	40 80 c6 fc                                     	add    $0xfc,%sil
  1d1d23:	75 db                                           	jne    1d1d00 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x9e0>
  1d1d25:	84 d2                                           	test   %dl,%dl
  1d1d27:	74 1d                                           	je     1d1d46 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa26>
  1d1d29:	fe c8                                           	dec    %al
  1d1d2b:	31 f6                                           	xor    %esi,%esi
  1d1d2d:	0f 1f 00                                        	nopl   (%rax)
  1d1d30:	89 c1                                           	mov    %eax,%ecx
  1d1d32:	80 e1 07                                        	and    $0x7,%cl
  1d1d35:	89 f7                                           	mov    %esi,%edi
  1d1d37:	40 d2 e7                                        	shl    %cl,%dil
  1d1d3a:	41 08 ff                                        	or     %dil,%r15b
  1d1d3d:	44 30 e6                                        	xor    %r12b,%sil
  1d1d40:	fe c8                                           	dec    %al
  1d1d42:	fe ca                                           	dec    %dl
  1d1d44:	75 ea                                           	jne    1d1d30 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa10>
  1d1d46:	4c 3b 6d 00                                     	cmp    0x0(%rbp),%r13
  1d1d4a:	75 0e                                           	jne    1d1d5a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa3a>
  1d1d4c:	48 89 ef                                        	mov    %rbp,%rdi
  1d1d4f:	ff 15 9b c0 09 00                               	call   *0x9c09b(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1d1d55:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d1d5a:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1d1d5e:	46 88 3c 28                                     	mov    %r15b,(%rax,%r13,1)
  1d1d62:	49 ff c5                                        	inc    %r13
  1d1d65:	4c 89 6d 10                                     	mov    %r13,0x10(%rbp)
  1d1d69:	41 80 ff ff                                     	cmp    $0xff,%r15b
  1d1d6d:	0f 94 c0                                        	sete   %al
  1d1d70:	b1 08                                           	mov    $0x8,%cl
  1d1d72:	28 c1                                           	sub    %al,%cl
  1d1d74:	88 4c 24 6b                                     	mov    %cl,0x6b(%rsp)
  1d1d78:	88 4c 24 6c                                     	mov    %cl,0x6c(%rsp)
  1d1d7c:	c6 44 24 6a 00                                  	movb   $0x0,0x6a(%rsp)
  1d1d81:	48 8b ac 24 e0 00 00 00                         	mov    0xe0(%rsp),%rbp
  1d1d89:	48 85 ed                                        	test   %rbp,%rbp
  1d1d8c:	74 2b                                           	je     1d1db9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa99>
  1d1d8e:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d1d93:	4c 8b 78 10                                     	mov    0x10(%rax),%r15
  1d1d97:	4c 2b 7c 24 28                                  	sub    0x28(%rsp),%r15
  1d1d9c:	4c 8b 6d 10                                     	mov    0x10(%rbp),%r13
  1d1da0:	4c 3b 6d 00                                     	cmp    0x0(%rbp),%r13
  1d1da4:	0f 84 9c 00 00 00                               	je     1d1e46 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xb26>
  1d1daa:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1d1dae:	4e 89 3c e8                                     	mov    %r15,(%rax,%r13,8)
  1d1db2:	49 ff c5                                        	inc    %r13
  1d1db5:	4c 89 6d 10                                     	mov    %r13,0x10(%rbp)
  1d1db9:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d1dbf:	4c 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%r15
  1d1dc7:	0f 83 33 fc ff ff                               	jae    1d1a00 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6e0>
  1d1dcd:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d1dd2:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d1dd6:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1d1ddb:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d1ddf:	0f 84 04 fc ff ff                               	je     1d19e9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6c9>
  1d1de5:	66 83 fb 09                                     	cmp    $0x9,%bx
  1d1de9:	0f 82 fa fb ff ff                               	jb     1d19e9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6c9>
  1d1def:	41 0f b7 c6                                     	movzwl %r14w,%eax
  1d1df3:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d1df9:	c1 e8 11                                        	shr    $0x11,%eax
  1d1dfc:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d1dff:	44 89 f1                                        	mov    %r14d,%ecx
  1d1e02:	29 c1                                           	sub    %eax,%ecx
  1d1e04:	66 85 c9                                        	test   %cx,%cx
  1d1e07:	0f 84 dc fb ff ff                               	je     1d19e9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6c9>
  1d1e0d:	66 c7 44 24 6a 00 08                            	movw   $0x800,0x6a(%rsp)
  1d1e14:	c6 44 24 6c 08                                  	movb   $0x8,0x6c(%rsp)
  1d1e19:	e9 e2 fb ff ff                                  	jmp    1d1a00 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x6e0>
  1d1e1e:	48 8b bc 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdi
  1d1e26:	4c 89 f0                                        	mov    %r14,%rax
  1d1e29:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d1e30:	4c 39 ff                                        	cmp    %r15,%rdi
  1d1e33:	0f 87 4b 0b 00 00                               	ja     1d2984 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1664>
  1d1e39:	4c 01 c7                                        	add    %r8,%rdi
  1d1e3c:	48 ff c8                                        	dec    %rax
  1d1e3f:	75 ef                                           	jne    1d1e30 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xb10>
  1d1e41:	e9 e5 fd ff ff                                  	jmp    1d1c2b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x90b>
  1d1e46:	48 89 ef                                        	mov    %rbp,%rdi
  1d1e49:	ff 15 41 c8 09 00                               	call   *0x9c841(%rip)        # 26e690 <_DYNAMIC+0xb10>
  1d1e4f:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d1e54:	e9 51 ff ff ff                                  	jmp    1d1daa <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xa8a>
  1d1e59:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
  1d1e5e:	48 8b 48 40                                     	mov    0x40(%rax),%rcx
  1d1e62:	48 89 8c 24 88 00 00 00                         	mov    %rcx,0x88(%rsp)
  1d1e6a:	4c 8b 70 38                                     	mov    0x38(%rax),%r14
  1d1e6e:	48 8b 08                                        	mov    (%rax),%rcx
  1d1e71:	48 89 8c 24 80 00 00 00                         	mov    %rcx,0x80(%rsp)
  1d1e79:	4c 8b 60 08                                     	mov    0x8(%rax),%r12
  1d1e7d:	48 8b 78 30                                     	mov    0x30(%rax),%rdi
  1d1e81:	48 89 7c 24 78                                  	mov    %rdi,0x78(%rsp)
  1d1e86:	48 8d 04 7f                                     	lea    (%rdi,%rdi,2),%rax
  1d1e8a:	48 83 c0 fd                                     	add    $0xfffffffffffffffd,%rax
  1d1e8e:	48 b9 ab aa aa aa aa aa aa aa                   	movabs $0xaaaaaaaaaaaaaaab,%rcx
  1d1e98:	48 89 84 24 b0 00 00 00                         	mov    %rax,0xb0(%rsp)
  1d1ea0:	48 f7 e1                                        	mul    %rcx
  1d1ea3:	48 8d 04 7f                                     	lea    (%rdi,%rdi,2),%rax
  1d1ea7:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
  1d1eaf:	48 d1 ea                                        	shr    $1,%rdx
  1d1eb2:	8d 42 01                                        	lea    0x1(%rdx),%eax
  1d1eb5:	83 e0 07                                        	and    $0x7,%eax
  1d1eb8:	83 e2 07                                        	and    $0x7,%edx
  1d1ebb:	48 89 94 24 a8 00 00 00                         	mov    %rdx,0xa8(%rsp)
  1d1ec3:	48 f7 d8                                        	neg    %rax
  1d1ec6:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
  1d1ece:	45 31 ff                                        	xor    %r15d,%r15d
  1d1ed1:	48 8d 84 24 f8 00 00 00                         	lea    0xf8(%rsp),%rax
  1d1ed9:	48 89 84 24 d8 00 00 00                         	mov    %rax,0xd8(%rsp)
  1d1ee1:	4c 89 a4 24 90 00 00 00                         	mov    %r12,0x90(%rsp)
  1d1ee9:	eb 25                                           	jmp    1d1f10 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbf0>
  1d1eeb:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d1ef4:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d1efc:	0f 1f 40 00                                     	nopl   0x0(%rax)
  1d1f00:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d1f06:	8b 74 24 74                                     	mov    0x74(%rsp),%esi
  1d1f0a:	0f 84 6a 09 00 00                               	je     1d287a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x155a>
  1d1f10:	41 0f b7 c7                                     	movzwl %r15w,%eax
  1d1f14:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d1f1a:	c1 e8 11                                        	shr    $0x11,%eax
  1d1f1d:	8d 0c 40                                        	lea    (%rax,%rax,2),%ecx
  1d1f20:	31 d2                                           	xor    %edx,%edx
  1d1f22:	44 89 fb                                        	mov    %r15d,%ebx
  1d1f25:	66 29 cb                                        	sub    %cx,%bx
  1d1f28:	0f 95 c2                                        	setne  %dl
  1d1f2b:	01 c2                                           	add    %eax,%edx
  1d1f2d:	b0 03                                           	mov    $0x3,%al
  1d1f2f:	89 f1                                           	mov    %esi,%ecx
  1d1f31:	28 d1                                           	sub    %dl,%cl
  1d1f33:	0f 82 30 09 00 00                               	jb     1d2869 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1549>
  1d1f39:	44 89 fd                                        	mov    %r15d,%ebp
  1d1f3c:	66 85 db                                        	test   %bx,%bx
  1d1f3f:	0f 95 c2                                        	setne  %dl
  1d1f42:	66 41 83 ff 0a                                  	cmp    $0xa,%r15w
  1d1f47:	41 0f 93 c5                                     	setae  %r13b
  1d1f4b:	45 20 d5                                        	and    %r10b,%r13b
  1d1f4e:	41 20 d5                                        	and    %dl,%r13b
  1d1f51:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  1d1f56:	88 4a 48                                        	mov    %cl,0x48(%rdx)
  1d1f59:	66 85 db                                        	test   %bx,%bx
  1d1f5c:	74 32                                           	je     1d1f90 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xc70>
  1d1f5e:	0f b7 c3                                        	movzwl %bx,%eax
  1d1f61:	83 f8 01                                        	cmp    $0x1,%eax
  1d1f64:	0f 85 86 01 00 00                               	jne    1d20f0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xdd0>
  1d1f6a:	45 84 ed                                        	test   %r13b,%r13b
  1d1f6d:	0f 84 93 01 00 00                               	je     1d2106 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xde6>
  1d1f73:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d1f78:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d1f7d:	e8 ae 87 fe ff                                  	call   1ba730 <emuella_j2k_tier1::significance_propagation_pass_encode_raw::<true>>
  1d1f82:	e9 9f 01 00 00                                  	jmp    1d2126 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe06>
  1d1f87:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d1f90:	45 84 ed                                        	test   %r13b,%r13b
  1d1f93:	4c 8b bc 24 88 00 00 00                         	mov    0x88(%rsp),%r15
  1d1f9b:	0f 85 15 09 00 00                               	jne    1d28b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1596>
  1d1fa1:	48 8d bc 24 b8 00 00 00                         	lea    0xb8(%rsp),%rdi
  1d1fa9:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1d1fae:	48 8d 54 24 28                                  	lea    0x28(%rsp),%rdx
  1d1fb3:	e8 88 ec fd ff                                  	call   1b0c40 <emuella_j2k_tier1::cleanup_pass_encode::<true>>
  1d1fb8:	0f b6 84 24 b8 00 00 00                         	movzbl 0xb8(%rsp),%eax
  1d1fc0:	3c ff                                           	cmp    $0xff,%al
  1d1fc2:	0f 85 3b 09 00 00                               	jne    1d2903 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x15e3>
  1d1fc8:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d1fcd:	41 f6 c2 20                                     	test   $0x20,%r10b
  1d1fd1:	74 4f                                           	je     1d2022 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd02>
  1d1fd3:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1fd8:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d1fdd:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d1fe2:	e8 19 7c 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d1fe7:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1fec:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d1ff1:	31 d2                                           	xor    %edx,%edx
  1d1ff3:	e8 08 7c 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d1ff8:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d1ffd:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d2002:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d2007:	e8 f4 7b 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d200c:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d2011:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d2016:	31 d2                                           	xor    %edx,%edx
  1d2018:	e8 e3 7b 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d201d:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d2022:	4d 85 ff                                        	test   %r15,%r15
  1d2025:	44 0f b6 44 24 20                               	movzbl 0x20(%rsp),%r8d
  1d202b:	4c 8b 8c 24 80 00 00 00                         	mov    0x80(%rsp),%r9
  1d2033:	4c 8b 5c 24 78                                  	mov    0x78(%rsp),%r11
  1d2038:	0f 84 f3 00 00 00                               	je     1d2131 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe11>
  1d203e:	4d 85 db                                        	test   %r11,%r11
  1d2041:	0f 84 c6 02 00 00                               	je     1d230d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xfed>
  1d2047:	31 c0                                           	xor    %eax,%eax
  1d2049:	eb 0e                                           	jmp    1d2059 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd39>
  1d204b:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
  1d2050:	4c 39 f8                                        	cmp    %r15,%rax
  1d2053:	0f 84 d8 00 00 00                               	je     1d2131 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe11>
  1d2059:	48 ff c0                                        	inc    %rax
  1d205c:	48 89 c7                                        	mov    %rax,%rdi
  1d205f:	49 0f af fe                                     	imul   %r14,%rdi
  1d2063:	48 ff c7                                        	inc    %rdi
  1d2066:	48 89 fe                                        	mov    %rdi,%rsi
  1d2069:	4c 01 de                                        	add    %r11,%rsi
  1d206c:	0f 82 3b 09 00 00                               	jb     1d29ad <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x168d>
  1d2072:	4c 39 e6                                        	cmp    %r12,%rsi
  1d2075:	0f 87 32 09 00 00                               	ja     1d29ad <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x168d>
  1d207b:	48 8d 0c 7f                                     	lea    (%rdi,%rdi,2),%rcx
  1d207f:	4c 01 c9                                        	add    %r9,%rcx
  1d2082:	48 89 ca                                        	mov    %rcx,%rdx
  1d2085:	83 bc 24 a8 00 00 00 07                         	cmpl   $0x7,0xa8(%rsp)
  1d208d:	74 1e                                           	je     1d20ad <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd8d>
  1d208f:	48 8b b4 24 98 00 00 00                         	mov    0x98(%rsp),%rsi
  1d2097:	48 89 ca                                        	mov    %rcx,%rdx
  1d209a:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d20a0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d20a4:	48 83 c2 03                                     	add    $0x3,%rdx
  1d20a8:	48 ff c6                                        	inc    %rsi
  1d20ab:	75 f3                                           	jne    1d20a0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd80>
  1d20ad:	48 83 bc 24 b0 00 00 00 15                      	cmpq   $0x15,0xb0(%rsp)
  1d20b6:	72 98                                           	jb     1d2050 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd30>
  1d20b8:	48 03 8c 24 a0 00 00 00                         	add    0xa0(%rsp),%rcx
  1d20c0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d20c4:	c6 42 04 00                                     	movb   $0x0,0x4(%rdx)
  1d20c8:	c6 42 07 00                                     	movb   $0x0,0x7(%rdx)
  1d20cc:	c6 42 0a 00                                     	movb   $0x0,0xa(%rdx)
  1d20d0:	c6 42 0d 00                                     	movb   $0x0,0xd(%rdx)
  1d20d4:	c6 42 10 00                                     	movb   $0x0,0x10(%rdx)
  1d20d8:	c6 42 13 00                                     	movb   $0x0,0x13(%rdx)
  1d20dc:	c6 42 16 00                                     	movb   $0x0,0x16(%rdx)
  1d20e0:	48 83 c2 18                                     	add    $0x18,%rdx
  1d20e4:	48 39 ca                                        	cmp    %rcx,%rdx
  1d20e7:	75 d7                                           	jne    1d20c0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xda0>
  1d20e9:	e9 62 ff ff ff                                  	jmp    1d2050 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xd30>
  1d20ee:	66 90                                           	xchg   %ax,%ax
  1d20f0:	45 84 ed                                        	test   %r13b,%r13b
  1d20f3:	74 22                                           	je     1d2117 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xdf7>
  1d20f5:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d20fa:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d20ff:	e8 bc 08 00 00                                  	call   1d29c0 <emuella_j2k_tier1::magnitude_refinement_pass_encode_raw>
  1d2104:	eb 20                                           	jmp    1d2126 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe06>
  1d2106:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d210b:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d2110:	e8 fb 6f fe ff                                  	call   1b9110 <emuella_j2k_tier1::significance_propagation_pass_encode::<true>>
  1d2115:	eb 0f                                           	jmp    1d2126 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe06>
  1d2117:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d211c:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d2121:	e8 da 4e fe ff                                  	call   1b7000 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<true>>
  1d2126:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d212b:	44 0f b6 44 24 20                               	movzbl 0x20(%rsp),%r8d
  1d2131:	44 8d 7d 01                                     	lea    0x1(%rbp),%r15d
  1d2135:	41 f6 c2 04                                     	test   $0x4,%r10b
  1d2139:	75 35                                           	jne    1d2170 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe50>
  1d213b:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d2141:	74 2d                                           	je     1d2170 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe50>
  1d2143:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d2147:	0f 84 b3 fd ff ff                               	je     1d1f00 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbe0>
  1d214d:	66 83 fd 09                                     	cmp    $0x9,%bp
  1d2151:	74 1d                                           	je     1d2170 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe50>
  1d2153:	0f 96 c0                                        	setbe  %al
  1d2156:	66 83 fb 01                                     	cmp    $0x1,%bx
  1d215a:	0f 94 c1                                        	sete   %cl
  1d215d:	08 c1                                           	or     %al,%cl
  1d215f:	0f 85 9b fd ff ff                               	jne    1d1f00 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbe0>
  1d2165:	66 66 2e 0f 1f 84 00 00 00 00 00                	data16 cs nopw 0x0(%rax,%rax,1)
  1d2170:	45 84 ed                                        	test   %r13b,%r13b
  1d2173:	74 3b                                           	je     1d21b0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe90>
  1d2175:	0f b6 44 24 6c                                  	movzbl 0x6c(%rsp),%eax
  1d217a:	0f b6 54 24 6b                                  	movzbl 0x6b(%rsp),%edx
  1d217f:	48 8b 4c 24 28                                  	mov    0x28(%rsp),%rcx
  1d2184:	48 8b 5c 24 30                                  	mov    0x30(%rsp),%rbx
  1d2189:	4c 8b 6b 10                                     	mov    0x10(%rbx),%r13
  1d218d:	49 39 cd                                        	cmp    %rcx,%r13
  1d2190:	76 36                                           	jbe    1d21c8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xea8>
  1d2192:	38 d0                                           	cmp    %dl,%al
  1d2194:	72 3a                                           	jb     1d21d0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xeb0>
  1d2196:	48 8b 53 08                                     	mov    0x8(%rbx),%rdx
  1d219a:	48 01 ca                                        	add    %rcx,%rdx
  1d219d:	4c 01 ea                                        	add    %r13,%rdx
  1d21a0:	48 f7 d1                                        	not    %rcx
  1d21a3:	80 3c 11 ff                                     	cmpb   $0xff,(%rcx,%rdx,1)
  1d21a7:	0f 85 c3 00 00 00                               	jne    1d2270 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf50>
  1d21ad:	eb 21                                           	jmp    1d21d0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xeb0>
  1d21af:	90                                              	nop
  1d21b0:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d21b5:	8b 74 24 70                                     	mov    0x70(%rsp),%esi
  1d21b9:	e8 82 e6 ff ff                                  	call   1d0840 <emuella_j2k_tier1::terminate_arithmetic_segment>
  1d21be:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d21c3:	e9 a8 00 00 00                                  	jmp    1d2270 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf50>
  1d21c8:	38 d0                                           	cmp    %dl,%al
  1d21ca:	0f 83 a0 00 00 00                               	jae    1d2270 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf50>
  1d21d0:	44 0f b6 64 24 6a                               	movzbl 0x6a(%rsp),%r12d
  1d21d6:	84 c0                                           	test   %al,%al
  1d21d8:	74 5c                                           	je     1d2236 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf16>
  1d21da:	89 c2                                           	mov    %eax,%edx
  1d21dc:	80 e2 03                                        	and    $0x3,%dl
  1d21df:	3c 04                                           	cmp    $0x4,%al
  1d21e1:	72 36                                           	jb     1d2219 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xef9>
  1d21e3:	89 c6                                           	mov    %eax,%esi
  1d21e5:	40 80 e6 fc                                     	and    $0xfc,%sil
  1d21e9:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d21f0:	8d 48 06                                        	lea    0x6(%rax),%ecx
  1d21f3:	80 e1 07                                        	and    $0x7,%cl
  1d21f6:	44 89 c7                                        	mov    %r8d,%edi
  1d21f9:	40 d2 e7                                        	shl    %cl,%dil
  1d21fc:	44 08 e7                                        	or     %r12b,%dil
  1d21ff:	04 fc                                           	add    $0xfc,%al
  1d2201:	89 c1                                           	mov    %eax,%ecx
  1d2203:	80 e1 07                                        	and    $0x7,%cl
  1d2206:	45 89 c4                                        	mov    %r8d,%r12d
  1d2209:	41 d2 e4                                        	shl    %cl,%r12b
  1d220c:	41 08 fc                                        	or     %dil,%r12b
  1d220f:	40 80 c6 fc                                     	add    $0xfc,%sil
  1d2213:	75 db                                           	jne    1d21f0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xed0>
  1d2215:	84 d2                                           	test   %dl,%dl
  1d2217:	74 1d                                           	je     1d2236 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf16>
  1d2219:	fe c8                                           	dec    %al
  1d221b:	31 f6                                           	xor    %esi,%esi
  1d221d:	0f 1f 00                                        	nopl   (%rax)
  1d2220:	89 c1                                           	mov    %eax,%ecx
  1d2222:	80 e1 07                                        	and    $0x7,%cl
  1d2225:	89 f7                                           	mov    %esi,%edi
  1d2227:	40 d2 e7                                        	shl    %cl,%dil
  1d222a:	41 08 fc                                        	or     %dil,%r12b
  1d222d:	44 30 c6                                        	xor    %r8b,%sil
  1d2230:	fe c8                                           	dec    %al
  1d2232:	fe ca                                           	dec    %dl
  1d2234:	75 ea                                           	jne    1d2220 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf00>
  1d2236:	4c 3b 2b                                        	cmp    (%rbx),%r13
  1d2239:	75 0e                                           	jne    1d2249 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf29>
  1d223b:	48 89 df                                        	mov    %rbx,%rdi
  1d223e:	ff 15 ac bb 09 00                               	call   *0x9bbac(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1d2244:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d2249:	48 8b 43 08                                     	mov    0x8(%rbx),%rax
  1d224d:	46 88 24 28                                     	mov    %r12b,(%rax,%r13,1)
  1d2251:	49 ff c5                                        	inc    %r13
  1d2254:	4c 89 6b 10                                     	mov    %r13,0x10(%rbx)
  1d2258:	41 80 fc ff                                     	cmp    $0xff,%r12b
  1d225c:	0f 94 c0                                        	sete   %al
  1d225f:	b1 08                                           	mov    $0x8,%cl
  1d2261:	28 c1                                           	sub    %al,%cl
  1d2263:	88 4c 24 6b                                     	mov    %cl,0x6b(%rsp)
  1d2267:	88 4c 24 6c                                     	mov    %cl,0x6c(%rsp)
  1d226b:	c6 44 24 6a 00                                  	movb   $0x0,0x6a(%rsp)
  1d2270:	4c 8b ac 24 e0 00 00 00                         	mov    0xe0(%rsp),%r13
  1d2278:	4d 85 ed                                        	test   %r13,%r13
  1d227b:	74 2b                                           	je     1d22a8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf88>
  1d227d:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d2282:	48 8b 58 10                                     	mov    0x10(%rax),%rbx
  1d2286:	48 2b 5c 24 28                                  	sub    0x28(%rsp),%rbx
  1d228b:	4d 8b 65 10                                     	mov    0x10(%r13),%r12
  1d228f:	4d 3b 65 00                                     	cmp    0x0(%r13),%r12
  1d2293:	0f 84 9d 00 00 00                               	je     1d2336 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1016>
  1d2299:	49 8b 45 08                                     	mov    0x8(%r13),%rax
  1d229d:	4a 89 1c e0                                     	mov    %rbx,(%rax,%r12,8)
  1d22a1:	49 ff c4                                        	inc    %r12
  1d22a4:	4d 89 65 10                                     	mov    %r12,0x10(%r13)
  1d22a8:	66 44 3b 7c 24 1c                               	cmp    0x1c(%rsp),%r15w
  1d22ae:	4c 8b a4 24 90 00 00 00                         	mov    0x90(%rsp),%r12
  1d22b6:	0f 83 44 fc ff ff                               	jae    1d1f00 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbe0>
  1d22bc:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d22c1:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d22c5:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1d22ca:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d22ce:	0f 84 17 fc ff ff                               	je     1d1eeb <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbcb>
  1d22d4:	66 83 fd 09                                     	cmp    $0x9,%bp
  1d22d8:	0f 82 0d fc ff ff                               	jb     1d1eeb <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbcb>
  1d22de:	41 0f b7 c7                                     	movzwl %r15w,%eax
  1d22e2:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d22e8:	c1 e8 11                                        	shr    $0x11,%eax
  1d22eb:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d22ee:	44 89 f9                                        	mov    %r15d,%ecx
  1d22f1:	29 c1                                           	sub    %eax,%ecx
  1d22f3:	66 85 c9                                        	test   %cx,%cx
  1d22f6:	0f 84 ef fb ff ff                               	je     1d1eeb <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbcb>
  1d22fc:	66 c7 44 24 6a 00 08                            	movw   $0x800,0x6a(%rsp)
  1d2303:	c6 44 24 6c 08                                  	movb   $0x8,0x6c(%rsp)
  1d2308:	e9 f3 fb ff ff                                  	jmp    1d1f00 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xbe0>
  1d230d:	49 8d 7e 01                                     	lea    0x1(%r14),%rdi
  1d2311:	4c 89 f8                                        	mov    %r15,%rax
  1d2314:	66 66 66 2e 0f 1f 84 00 00 00 00 00             	data16 data16 cs nopw 0x0(%rax,%rax,1)
  1d2320:	4c 39 e7                                        	cmp    %r12,%rdi
  1d2323:	0f 87 81 06 00 00                               	ja     1d29aa <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x168a>
  1d2329:	4c 01 f7                                        	add    %r14,%rdi
  1d232c:	48 ff c8                                        	dec    %rax
  1d232f:	75 ef                                           	jne    1d2320 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1000>
  1d2331:	e9 fb fd ff ff                                  	jmp    1d2131 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xe11>
  1d2336:	4c 89 ef                                        	mov    %r13,%rdi
  1d2339:	ff 15 51 c3 09 00                               	call   *0x9c351(%rip)        # 26e690 <_DYNAMIC+0xb10>
  1d233f:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d2344:	e9 50 ff ff ff                                  	jmp    1d2299 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0xf79>
  1d2349:	48 8b 44 24 10                                  	mov    0x10(%rsp),%rax
  1d234e:	48 8b 48 40                                     	mov    0x40(%rax),%rcx
  1d2352:	48 89 8c 24 90 00 00 00                         	mov    %rcx,0x90(%rsp)
  1d235a:	48 8b 78 38                                     	mov    0x38(%rax),%rdi
  1d235e:	48 89 bc 24 88 00 00 00                         	mov    %rdi,0x88(%rsp)
  1d2366:	48 8b 08                                        	mov    (%rax),%rcx
  1d2369:	48 89 8c 24 80 00 00 00                         	mov    %rcx,0x80(%rsp)
  1d2371:	4c 8b 78 08                                     	mov    0x8(%rax),%r15
  1d2375:	48 8b 40 30                                     	mov    0x30(%rax),%rax
  1d2379:	48 89 44 24 78                                  	mov    %rax,0x78(%rsp)
  1d237e:	48 8d 0c 40                                     	lea    (%rax,%rax,2),%rcx
  1d2382:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  1d2387:	45 89 d4                                        	mov    %r10d,%r12d
  1d238a:	41 c0 ec 04                                     	shr    $0x4,%r12b
  1d238e:	41 80 e4 01                                     	and    $0x1,%r12b
  1d2392:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
  1d2396:	48 83 c0 fd                                     	add    $0xfffffffffffffffd,%rax
  1d239a:	48 b9 ab aa aa aa aa aa aa aa                   	movabs $0xaaaaaaaaaaaaaaab,%rcx
  1d23a4:	48 89 84 24 b0 00 00 00                         	mov    %rax,0xb0(%rsp)
  1d23ac:	48 f7 e1                                        	mul    %rcx
  1d23af:	48 d1 ea                                        	shr    $1,%rdx
  1d23b2:	8d 42 01                                        	lea    0x1(%rdx),%eax
  1d23b5:	83 e0 07                                        	and    $0x7,%eax
  1d23b8:	83 e2 07                                        	and    $0x7,%edx
  1d23bb:	48 89 94 24 a8 00 00 00                         	mov    %rdx,0xa8(%rsp)
  1d23c3:	48 f7 d8                                        	neg    %rax
  1d23c6:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
  1d23ce:	48 8d 47 01                                     	lea    0x1(%rdi),%rax
  1d23d2:	48 89 84 24 d0 00 00 00                         	mov    %rax,0xd0(%rsp)
  1d23da:	45 31 f6                                        	xor    %r14d,%r14d
  1d23dd:	48 8d 84 24 00 01 00 00                         	lea    0x100(%rsp),%rax
  1d23e5:	48 89 84 24 d8 00 00 00                         	mov    %rax,0xd8(%rsp)
  1d23ed:	4c 89 bc 24 98 00 00 00                         	mov    %r15,0x98(%rsp)
  1d23f5:	eb 29                                           	jmp    1d2420 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1100>
  1d23f7:	48 c7 44 24 38 00 80 00 00                      	movq   $0x8000,0x38(%rsp)
  1d2400:	c7 44 24 40 0c 00 00 00                         	movl   $0xc,0x40(%rsp)
  1d2408:	0f 1f 84 00 00 00 00 00                         	nopl   0x0(%rax,%rax,1)
  1d2410:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d2416:	8b 74 24 74                                     	mov    0x74(%rsp),%esi
  1d241a:	0f 84 5a 04 00 00                               	je     1d287a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x155a>
  1d2420:	41 0f b7 c6                                     	movzwl %r14w,%eax
  1d2424:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d242a:	c1 e8 11                                        	shr    $0x11,%eax
  1d242d:	8d 0c 40                                        	lea    (%rax,%rax,2),%ecx
  1d2430:	31 d2                                           	xor    %edx,%edx
  1d2432:	44 89 f5                                        	mov    %r14d,%ebp
  1d2435:	66 29 cd                                        	sub    %cx,%bp
  1d2438:	0f 95 c2                                        	setne  %dl
  1d243b:	01 c2                                           	add    %eax,%edx
  1d243d:	b0 03                                           	mov    $0x3,%al
  1d243f:	89 f1                                           	mov    %esi,%ecx
  1d2441:	28 d1                                           	sub    %dl,%cl
  1d2443:	0f 82 20 04 00 00                               	jb     1d2869 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1549>
  1d2449:	44 89 f3                                        	mov    %r14d,%ebx
  1d244c:	66 85 ed                                        	test   %bp,%bp
  1d244f:	0f 95 c2                                        	setne  %dl
  1d2452:	66 41 83 fe 0a                                  	cmp    $0xa,%r14w
  1d2457:	41 0f 93 c5                                     	setae  %r13b
  1d245b:	45 20 d5                                        	and    %r10b,%r13b
  1d245e:	41 20 d5                                        	and    %dl,%r13b
  1d2461:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
  1d2466:	88 4a 48                                        	mov    %cl,0x48(%rdx)
  1d2469:	66 85 ed                                        	test   %bp,%bp
  1d246c:	74 32                                           	je     1d24a0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1180>
  1d246e:	0f b7 c5                                        	movzwl %bp,%eax
  1d2471:	83 f8 01                                        	cmp    $0x1,%eax
  1d2474:	0f 85 86 01 00 00                               	jne    1d2600 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x12e0>
  1d247a:	45 84 ed                                        	test   %r13b,%r13b
  1d247d:	0f 84 93 01 00 00                               	je     1d2616 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x12f6>
  1d2483:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d2488:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d248d:	e8 9e 82 fe ff                                  	call   1ba730 <emuella_j2k_tier1::significance_propagation_pass_encode_raw::<true>>
  1d2492:	e9 9f 01 00 00                                  	jmp    1d2636 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1316>
  1d2497:	66 0f 1f 84 00 00 00 00 00                      	nopw   0x0(%rax,%rax,1)
  1d24a0:	45 84 ed                                        	test   %r13b,%r13b
  1d24a3:	4c 8b b4 24 90 00 00 00                         	mov    0x90(%rsp),%r14
  1d24ab:	0f 85 05 04 00 00                               	jne    1d28b6 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1596>
  1d24b1:	48 8d bc 24 b8 00 00 00                         	lea    0xb8(%rsp),%rdi
  1d24b9:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
  1d24be:	48 8d 54 24 28                                  	lea    0x28(%rsp),%rdx
  1d24c3:	e8 78 e7 fd ff                                  	call   1b0c40 <emuella_j2k_tier1::cleanup_pass_encode::<true>>
  1d24c8:	0f b6 84 24 b8 00 00 00                         	movzbl 0xb8(%rsp),%eax
  1d24d0:	3c ff                                           	cmp    $0xff,%al
  1d24d2:	0f 85 49 04 00 00                               	jne    1d2921 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1601>
  1d24d8:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d24dd:	41 f6 c2 20                                     	test   $0x20,%r10b
  1d24e1:	74 4f                                           	je     1d2532 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1212>
  1d24e3:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d24e8:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d24ed:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d24f2:	e8 09 77 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d24f7:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d24fc:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d2501:	31 d2                                           	xor    %edx,%edx
  1d2503:	e8 f8 76 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d2508:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d250d:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d2512:	ba 01 00 00 00                                  	mov    $0x1,%edx
  1d2517:	e8 e4 76 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d251c:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d2521:	be 12 00 00 00                                  	mov    $0x12,%esi
  1d2526:	31 d2                                           	xor    %edx,%edx
  1d2528:	e8 d3 76 00 00                                  	call   1d9c00 <<emuella_j2k_tier1::mq::Encoder>::write_bit>
  1d252d:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d2532:	4d 85 f6                                        	test   %r14,%r14
  1d2535:	4c 8b 84 24 88 00 00 00                         	mov    0x88(%rsp),%r8
  1d253d:	4c 8b 8c 24 80 00 00 00                         	mov    0x80(%rsp),%r9
  1d2545:	4c 8b 5c 24 78                                  	mov    0x78(%rsp),%r11
  1d254a:	0f 84 eb 00 00 00                               	je     1d263b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x131b>
  1d2550:	4d 85 db                                        	test   %r11,%r11
  1d2553:	0f 84 d5 02 00 00                               	je     1d282e <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x150e>
  1d2559:	31 c0                                           	xor    %eax,%eax
  1d255b:	eb 0c                                           	jmp    1d2569 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1249>
  1d255d:	0f 1f 00                                        	nopl   (%rax)
  1d2560:	4c 39 f0                                        	cmp    %r14,%rax
  1d2563:	0f 84 d2 00 00 00                               	je     1d263b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x131b>
  1d2569:	48 ff c0                                        	inc    %rax
  1d256c:	48 89 c7                                        	mov    %rax,%rdi
  1d256f:	49 0f af f8                                     	imul   %r8,%rdi
  1d2573:	48 ff c7                                        	inc    %rdi
  1d2576:	48 89 fe                                        	mov    %rdi,%rsi
  1d2579:	4c 01 de                                        	add    %r11,%rsi
  1d257c:	0f 82 05 04 00 00                               	jb     1d2987 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1667>
  1d2582:	4c 39 fe                                        	cmp    %r15,%rsi
  1d2585:	0f 87 fc 03 00 00                               	ja     1d2987 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1667>
  1d258b:	48 8d 0c 7f                                     	lea    (%rdi,%rdi,2),%rcx
  1d258f:	4c 01 c9                                        	add    %r9,%rcx
  1d2592:	48 89 ca                                        	mov    %rcx,%rdx
  1d2595:	83 bc 24 a8 00 00 00 07                         	cmpl   $0x7,0xa8(%rsp)
  1d259d:	74 1e                                           	je     1d25bd <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x129d>
  1d259f:	48 8b b4 24 a0 00 00 00                         	mov    0xa0(%rsp),%rsi
  1d25a7:	48 89 ca                                        	mov    %rcx,%rdx
  1d25aa:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
  1d25b0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d25b4:	48 83 c2 03                                     	add    $0x3,%rdx
  1d25b8:	48 ff c6                                        	inc    %rsi
  1d25bb:	75 f3                                           	jne    1d25b0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1290>
  1d25bd:	48 83 bc 24 b0 00 00 00 15                      	cmpq   $0x15,0xb0(%rsp)
  1d25c6:	72 98                                           	jb     1d2560 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1240>
  1d25c8:	48 03 4c 24 20                                  	add    0x20(%rsp),%rcx
  1d25cd:	0f 1f 00                                        	nopl   (%rax)
  1d25d0:	c6 42 01 00                                     	movb   $0x0,0x1(%rdx)
  1d25d4:	c6 42 04 00                                     	movb   $0x0,0x4(%rdx)
  1d25d8:	c6 42 07 00                                     	movb   $0x0,0x7(%rdx)
  1d25dc:	c6 42 0a 00                                     	movb   $0x0,0xa(%rdx)
  1d25e0:	c6 42 0d 00                                     	movb   $0x0,0xd(%rdx)
  1d25e4:	c6 42 10 00                                     	movb   $0x0,0x10(%rdx)
  1d25e8:	c6 42 13 00                                     	movb   $0x0,0x13(%rdx)
  1d25ec:	c6 42 16 00                                     	movb   $0x0,0x16(%rdx)
  1d25f0:	48 83 c2 18                                     	add    $0x18,%rdx
  1d25f4:	48 39 ca                                        	cmp    %rcx,%rdx
  1d25f7:	75 d7                                           	jne    1d25d0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x12b0>
  1d25f9:	e9 62 ff ff ff                                  	jmp    1d2560 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1240>
  1d25fe:	66 90                                           	xchg   %ax,%ax
  1d2600:	45 84 ed                                        	test   %r13b,%r13b
  1d2603:	74 22                                           	je     1d2627 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1307>
  1d2605:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d260a:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d260f:	e8 ac 03 00 00                                  	call   1d29c0 <emuella_j2k_tier1::magnitude_refinement_pass_encode_raw>
  1d2614:	eb 20                                           	jmp    1d2636 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1316>
  1d2616:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d261b:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d2620:	e8 eb 6a fe ff                                  	call   1b9110 <emuella_j2k_tier1::significance_propagation_pass_encode::<true>>
  1d2625:	eb 0f                                           	jmp    1d2636 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1316>
  1d2627:	48 8b 7c 24 10                                  	mov    0x10(%rsp),%rdi
  1d262c:	48 8d 74 24 28                                  	lea    0x28(%rsp),%rsi
  1d2631:	e8 ca 49 fe ff                                  	call   1b7000 <emuella_j2k_tier1::magnitude_refinement_pass_encode::<true>>
  1d2636:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d263b:	44 8d 73 01                                     	lea    0x1(%rbx),%r14d
  1d263f:	c6 44 24 44 04                                  	movb   $0x4,0x44(%rsp)
  1d2644:	48 8d 44 24 45                                  	lea    0x45(%rsp),%rax
  1d2649:	0f 57 c0                                        	xorps  %xmm0,%xmm0
  1d264c:	0f 11 40 10                                     	movups %xmm0,0x10(%rax)
  1d2650:	0f 11 00                                        	movups %xmm0,(%rax)
  1d2653:	c6 40 20 00                                     	movb   $0x0,0x20(%rax)
  1d2657:	c7 44 24 66 03 00 2e 00                         	movl   $0x2e0003,0x66(%rsp)
  1d265f:	41 f6 c2 04                                     	test   $0x4,%r10b
  1d2663:	75 2b                                           	jne    1d2690 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1370>
  1d2665:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d266b:	74 23                                           	je     1d2690 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1370>
  1d266d:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d2671:	0f 84 99 fd ff ff                               	je     1d2410 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10f0>
  1d2677:	66 83 fb 09                                     	cmp    $0x9,%bx
  1d267b:	74 13                                           	je     1d2690 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1370>
  1d267d:	0f 96 c0                                        	setbe  %al
  1d2680:	66 83 fd 01                                     	cmp    $0x1,%bp
  1d2684:	0f 94 c1                                        	sete   %cl
  1d2687:	08 c1                                           	or     %al,%cl
  1d2689:	0f 85 81 fd ff ff                               	jne    1d2410 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10f0>
  1d268f:	90                                              	nop
  1d2690:	45 84 ed                                        	test   %r13b,%r13b
  1d2693:	74 3b                                           	je     1d26d0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13b0>
  1d2695:	0f b6 44 24 6c                                  	movzbl 0x6c(%rsp),%eax
  1d269a:	0f b6 54 24 6b                                  	movzbl 0x6b(%rsp),%edx
  1d269f:	48 8b 4c 24 28                                  	mov    0x28(%rsp),%rcx
  1d26a4:	48 8b 6c 24 30                                  	mov    0x30(%rsp),%rbp
  1d26a9:	4c 8b 6d 10                                     	mov    0x10(%rbp),%r13
  1d26ad:	49 39 cd                                        	cmp    %rcx,%r13
  1d26b0:	76 36                                           	jbe    1d26e8 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13c8>
  1d26b2:	38 d0                                           	cmp    %dl,%al
  1d26b4:	72 3a                                           	jb     1d26f0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13d0>
  1d26b6:	48 8b 55 08                                     	mov    0x8(%rbp),%rdx
  1d26ba:	48 01 ca                                        	add    %rcx,%rdx
  1d26bd:	4c 01 ea                                        	add    %r13,%rdx
  1d26c0:	48 f7 d1                                        	not    %rcx
  1d26c3:	80 3c 11 ff                                     	cmpb   $0xff,(%rcx,%rdx,1)
  1d26c7:	0f 85 c4 00 00 00                               	jne    1d2791 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1471>
  1d26cd:	eb 21                                           	jmp    1d26f0 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13d0>
  1d26cf:	90                                              	nop
  1d26d0:	48 8d 7c 24 28                                  	lea    0x28(%rsp),%rdi
  1d26d5:	8b 74 24 70                                     	mov    0x70(%rsp),%esi
  1d26d9:	e8 62 e1 ff ff                                  	call   1d0840 <emuella_j2k_tier1::terminate_arithmetic_segment>
  1d26de:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d26e3:	e9 a9 00 00 00                                  	jmp    1d2791 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1471>
  1d26e8:	38 d0                                           	cmp    %dl,%al
  1d26ea:	0f 83 a1 00 00 00                               	jae    1d2791 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1471>
  1d26f0:	44 0f b6 7c 24 6a                               	movzbl 0x6a(%rsp),%r15d
  1d26f6:	84 c0                                           	test   %al,%al
  1d26f8:	74 5c                                           	je     1d2756 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1436>
  1d26fa:	89 c2                                           	mov    %eax,%edx
  1d26fc:	80 e2 03                                        	and    $0x3,%dl
  1d26ff:	3c 04                                           	cmp    $0x4,%al
  1d2701:	72 36                                           	jb     1d2739 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1419>
  1d2703:	89 c6                                           	mov    %eax,%esi
  1d2705:	40 80 e6 fc                                     	and    $0xfc,%sil
  1d2709:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d2710:	8d 48 06                                        	lea    0x6(%rax),%ecx
  1d2713:	80 e1 07                                        	and    $0x7,%cl
  1d2716:	44 89 e7                                        	mov    %r12d,%edi
  1d2719:	40 d2 e7                                        	shl    %cl,%dil
  1d271c:	44 08 ff                                        	or     %r15b,%dil
  1d271f:	04 fc                                           	add    $0xfc,%al
  1d2721:	89 c1                                           	mov    %eax,%ecx
  1d2723:	80 e1 07                                        	and    $0x7,%cl
  1d2726:	45 89 e7                                        	mov    %r12d,%r15d
  1d2729:	41 d2 e7                                        	shl    %cl,%r15b
  1d272c:	41 08 ff                                        	or     %dil,%r15b
  1d272f:	40 80 c6 fc                                     	add    $0xfc,%sil
  1d2733:	75 db                                           	jne    1d2710 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x13f0>
  1d2735:	84 d2                                           	test   %dl,%dl
  1d2737:	74 1d                                           	je     1d2756 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1436>
  1d2739:	fe c8                                           	dec    %al
  1d273b:	31 f6                                           	xor    %esi,%esi
  1d273d:	0f 1f 00                                        	nopl   (%rax)
  1d2740:	89 c1                                           	mov    %eax,%ecx
  1d2742:	80 e1 07                                        	and    $0x7,%cl
  1d2745:	89 f7                                           	mov    %esi,%edi
  1d2747:	40 d2 e7                                        	shl    %cl,%dil
  1d274a:	41 08 ff                                        	or     %dil,%r15b
  1d274d:	44 30 e6                                        	xor    %r12b,%sil
  1d2750:	fe c8                                           	dec    %al
  1d2752:	fe ca                                           	dec    %dl
  1d2754:	75 ea                                           	jne    1d2740 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1420>
  1d2756:	4c 3b 6d 00                                     	cmp    0x0(%rbp),%r13
  1d275a:	75 0e                                           	jne    1d276a <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x144a>
  1d275c:	48 89 ef                                        	mov    %rbp,%rdi
  1d275f:	ff 15 8b b6 09 00                               	call   *0x9b68b(%rip)        # 26ddf0 <_DYNAMIC+0x270>
  1d2765:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d276a:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1d276e:	46 88 3c 28                                     	mov    %r15b,(%rax,%r13,1)
  1d2772:	49 ff c5                                        	inc    %r13
  1d2775:	4c 89 6d 10                                     	mov    %r13,0x10(%rbp)
  1d2779:	41 80 ff ff                                     	cmp    $0xff,%r15b
  1d277d:	0f 94 c0                                        	sete   %al
  1d2780:	b1 08                                           	mov    $0x8,%cl
  1d2782:	28 c1                                           	sub    %al,%cl
  1d2784:	88 4c 24 6b                                     	mov    %cl,0x6b(%rsp)
  1d2788:	88 4c 24 6c                                     	mov    %cl,0x6c(%rsp)
  1d278c:	c6 44 24 6a 00                                  	movb   $0x0,0x6a(%rsp)
  1d2791:	48 8b ac 24 e0 00 00 00                         	mov    0xe0(%rsp),%rbp
  1d2799:	48 85 ed                                        	test   %rbp,%rbp
  1d279c:	74 2b                                           	je     1d27c9 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x14a9>
  1d279e:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d27a3:	4c 8b 78 10                                     	mov    0x10(%rax),%r15
  1d27a7:	4c 2b 7c 24 28                                  	sub    0x28(%rsp),%r15
  1d27ac:	4c 8b 6d 10                                     	mov    0x10(%rbp),%r13
  1d27b0:	4c 3b 6d 00                                     	cmp    0x0(%rbp),%r13
  1d27b4:	0f 84 9c 00 00 00                               	je     1d2856 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1536>
  1d27ba:	48 8b 45 08                                     	mov    0x8(%rbp),%rax
  1d27be:	4e 89 3c e8                                     	mov    %r15,(%rax,%r13,8)
  1d27c2:	49 ff c5                                        	inc    %r13
  1d27c5:	4c 89 6d 10                                     	mov    %r13,0x10(%rbp)
  1d27c9:	66 44 3b 74 24 1c                               	cmp    0x1c(%rsp),%r14w
  1d27cf:	4c 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%r15
  1d27d7:	0f 83 33 fc ff ff                               	jae    1d2410 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10f0>
  1d27dd:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d27e2:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d27e6:	48 89 44 24 28                                  	mov    %rax,0x28(%rsp)
  1d27eb:	41 f6 c2 01                                     	test   $0x1,%r10b
  1d27ef:	0f 84 02 fc ff ff                               	je     1d23f7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10d7>
  1d27f5:	66 83 fb 09                                     	cmp    $0x9,%bx
  1d27f9:	0f 82 f8 fb ff ff                               	jb     1d23f7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10d7>
  1d27ff:	41 0f b7 c6                                     	movzwl %r14w,%eax
  1d2803:	69 c0 ab aa 00 00                               	imul   $0xaaab,%eax,%eax
  1d2809:	c1 e8 11                                        	shr    $0x11,%eax
  1d280c:	8d 04 40                                        	lea    (%rax,%rax,2),%eax
  1d280f:	44 89 f1                                        	mov    %r14d,%ecx
  1d2812:	29 c1                                           	sub    %eax,%ecx
  1d2814:	66 85 c9                                        	test   %cx,%cx
  1d2817:	0f 84 da fb ff ff                               	je     1d23f7 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10d7>
  1d281d:	66 c7 44 24 6a 00 08                            	movw   $0x800,0x6a(%rsp)
  1d2824:	c6 44 24 6c 08                                  	movb   $0x8,0x6c(%rsp)
  1d2829:	e9 e2 fb ff ff                                  	jmp    1d2410 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x10f0>
  1d282e:	48 8b bc 24 d0 00 00 00                         	mov    0xd0(%rsp),%rdi
  1d2836:	4c 89 f0                                        	mov    %r14,%rax
  1d2839:	0f 1f 80 00 00 00 00                            	nopl   0x0(%rax)
  1d2840:	4c 39 ff                                        	cmp    %r15,%rdi
  1d2843:	0f 87 3b 01 00 00                               	ja     1d2984 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1664>
  1d2849:	4c 01 c7                                        	add    %r8,%rdi
  1d284c:	48 ff c8                                        	dec    %rax
  1d284f:	75 ef                                           	jne    1d2840 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1520>
  1d2851:	e9 e5 fd ff ff                                  	jmp    1d263b <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x131b>
  1d2856:	48 89 ef                                        	mov    %rbp,%rdi
  1d2859:	ff 15 31 be 09 00                               	call   *0x9be31(%rip)        # 26e690 <_DYNAMIC+0xb10>
  1d285f:	4c 8b 54 24 08                                  	mov    0x8(%rsp),%r10
  1d2864:	e9 51 ff ff ff                                  	jmp    1d27ba <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x149a>
  1d2869:	b9 28 00 00 00                                  	mov    $0x28,%ecx
  1d286e:	48 8d 3d 9d f6 e4 ff                            	lea    -0x1b0963(%rip),%rdi        # 21f12 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x4672>
  1d2875:	e9 d3 00 00 00                                  	jmp    1d294d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x162d>
  1d287a:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1d287f:	48 8b 40 10                                     	mov    0x10(%rax),%rax
  1d2883:	48 2b 84 24 18 01 00 00                         	sub    0x118(%rsp),%rax
  1d288b:	48 8b 8c 24 08 01 00 00                         	mov    0x108(%rsp),%rcx
  1d2893:	48 89 41 08                                     	mov    %rax,0x8(%rcx)
  1d2897:	8b 44 24 1c                                     	mov    0x1c(%rsp),%eax
  1d289b:	66 89 41 10                                     	mov    %ax,0x10(%rcx)
  1d289f:	48 8b 84 24 20 01 00 00                         	mov    0x120(%rsp),%rax
  1d28a7:	88 41 12                                        	mov    %al,0x12(%rcx)
  1d28aa:	c6 41 13 01                                     	movb   $0x1,0x13(%rcx)
  1d28ae:	c6 01 ff                                        	movb   $0xff,(%rcx)
  1d28b1:	e9 bc 00 00 00                                  	jmp    1d2972 <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x1652>
  1d28b6:	b9 2a 00 00 00                                  	mov    $0x2a,%ecx
  1d28bb:	48 8d 3d 26 f6 e4 ff                            	lea    -0x1b09da(%rip),%rdi        # 21ee8 <anon.bd84da5dcd26c1be380e6d835091d5c8.1255.llvm.4847318650076708598+0x4648>
  1d28c2:	e9 86 00 00 00                                  	jmp    1d294d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x162d>
  1d28c7:	8b 8c 24 b9 00 00 00                            	mov    0xb9(%rsp),%ecx
  1d28ce:	8b 94 24 bc 00 00 00                            	mov    0xbc(%rsp),%edx
  1d28d5:	89 94 24 eb 00 00 00                            	mov    %edx,0xeb(%rsp)
  1d28dc:	89 8c 24 e8 00 00 00                            	mov    %ecx,0xe8(%rsp)
  1d28e3:	eb 58                                           	jmp    1d293d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x161d>
  1d28e5:	8b 8c 24 b9 00 00 00                            	mov    0xb9(%rsp),%ecx
  1d28ec:	8b 94 24 bc 00 00 00                            	mov    0xbc(%rsp),%edx
  1d28f3:	89 94 24 f3 00 00 00                            	mov    %edx,0xf3(%rsp)
  1d28fa:	89 8c 24 f0 00 00 00                            	mov    %ecx,0xf0(%rsp)
  1d2901:	eb 3a                                           	jmp    1d293d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x161d>
  1d2903:	8b 8c 24 b9 00 00 00                            	mov    0xb9(%rsp),%ecx
  1d290a:	8b 94 24 bc 00 00 00                            	mov    0xbc(%rsp),%edx
  1d2911:	89 94 24 fb 00 00 00                            	mov    %edx,0xfb(%rsp)
  1d2918:	89 8c 24 f8 00 00 00                            	mov    %ecx,0xf8(%rsp)
  1d291f:	eb 1c                                           	jmp    1d293d <emuella_j2k_tier1::encode_prepared_baseline_code_block+0x161d>
  1d2921:	8b 8c 24 b9 00 00 00                            	mov    0xb9(%rsp),%ecx
  1d2928:	8b 94 24 bc 00 00 00                            	mov    0xbc(%rsp),%edx
  1d292f:	89 94 24 03 01 00 00                            	mov    %edx,0x103(%rsp)
  1d2936:	89 8c 24 00 01 00 00                            	mov    %ecx,0x100(%rsp)
  1d293d:	48 8b bc 24 c0 00 00 00                         	mov    0xc0(%rsp),%rdi
  1d2945:	48 8b 8c 24 c8 00 00 00                         	mov    0xc8(%rsp),%rcx
  1d294d:	48 8b b4 24 08 01 00 00                         	mov    0x108(%rsp),%rsi
  1d2955:	88 06                                           	mov    %al,(%rsi)
  1d2957:	48 8b 94 24 d8 00 00 00                         	mov    0xd8(%rsp),%rdx
  1d295f:	8b 02                                           	mov    (%rdx),%eax
  1d2961:	8b 52 03                                        	mov    0x3(%rdx),%edx
  1d2964:	89 46 01                                        	mov    %eax,0x1(%rsi)
  1d2967:	89 56 04                                        	mov    %edx,0x4(%rsi)
  1d296a:	48 89 7e 08                                     	mov    %rdi,0x8(%rsi)
  1d296e:	48 89 4e 10                                     	mov    %rcx,0x10(%rsi)
  1d2972:	48 81 c4 28 01 00 00                            	add    $0x128,%rsp
  1d2979:	5b                                              	pop    %rbx
  1d297a:	41 5c                                           	pop    %r12
  1d297c:	41 5d                                           	pop    %r13
  1d297e:	41 5e                                           	pop    %r14
  1d2980:	41 5f                                           	pop    %r15
  1d2982:	5d                                              	pop    %rbp
  1d2983:	c3                                              	ret
  1d2984:	48 89 fe                                        	mov    %rdi,%rsi
  1d2987:	48 8d 0d 52 79 09 00                            	lea    0x97952(%rip),%rcx        # 26a2e0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3338>
  1d298e:	4c 89 fa                                        	mov    %r15,%rdx
  1d2991:	ff 15 59 b6 09 00                               	call   *0x9b659(%rip)        # 26dff0 <_DYNAMIC+0x470>
  1d2997:	48 89 fe                                        	mov    %rdi,%rsi
  1d299a:	48 8d 0d 3f 79 09 00                            	lea    0x9793f(%rip),%rcx        # 26a2e0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3338>
  1d29a1:	4c 89 f2                                        	mov    %r14,%rdx
  1d29a4:	ff 15 46 b6 09 00                               	call   *0x9b646(%rip)        # 26dff0 <_DYNAMIC+0x470>
  1d29aa:	48 89 fe                                        	mov    %rdi,%rsi
  1d29ad:	48 8d 0d 2c 79 09 00                            	lea    0x9792c(%rip),%rcx        # 26a2e0 <anon.554af9682b4dcd8c7396a990a1c31539.8.llvm.5651600593056820511+0x3338>
  1d29b4:	4c 89 e2                                        	mov    %r12,%rdx
  1d29b7:	ff 15 33 b6 09 00                               	call   *0x9b633(%rip)        # 26dff0 <_DYNAMIC+0x470>
