import { ellipsify } from '@/lib/utils'
import { ExplorerLink } from '../cluster/cluster-ui'
import { useWallet } from '@solana/wallet-adapter-react'

export default function BlessTimeFeature() {
  const { publicKey } = useWallet()
  const address = publicKey ?? undefined
  if (!address) {
    return <div className="flex-row flex justify-center py-[26px]">Please login the solana wallet first.</div>
  }

  return (
    <>
      <div className="justify-center flex-row">
        <div className="my-4 flex flex-row justify-center py-[16px] md:py-[64px]">
          <span style={{ fontSize: 15, color: '#0000ff' }}>Account address: &nbsp;&nbsp;&nbsp;</span>
          <ExplorerLink path={`account/${address}`} label={ellipsify(address.toString())} />
        </div>
        <div className="my-4 flex flex-row justify-center py-[16px] md:py-[64px]"></div>
      </div>
    </>
  )
}
