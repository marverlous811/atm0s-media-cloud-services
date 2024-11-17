import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { SidebarMenu, SidebarMenuButton, SidebarMenuItem, useSidebar } from '@/components/ui/sidebar'
import { Theme, useTheme } from '@/providers'
import { useClerk, useUser } from '@clerk/clerk-react'
import { BadgeCheck, Bell, ChevronsUpDown, LogOut, SunMoonIcon } from 'lucide-react'

export const NavUser = () => {
  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <UserInfo />
      </SidebarMenuItem>
    </SidebarMenu>
  )
}

type UserInfoProps = object

export const UserInfo: React.FC<UserInfoProps> = () => {
  const { theme, setTheme } = useTheme()
  const { isMobile } = useSidebar()
  const { signOut } = useClerk()

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <SidebarMenuButton
          size="lg"
          className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
        >
          <UserAvatar hasArrow />
        </SidebarMenuButton>
      </DropdownMenuTrigger>
      <DropdownMenuContent
        className="w-[--radix-dropdown-menu-trigger-width] min-w-56 rounded-lg"
        side={isMobile ? 'bottom' : 'right'}
        align="end"
        sideOffset={4}
      >
        <DropdownMenuLabel className="p-0 font-normal">
          <div className="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
            <UserAvatar />
          </div>
        </DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem>
            <BadgeCheck />
            Account
          </DropdownMenuItem>
          <DropdownMenuItem>
            <Bell />
            Notifications
          </DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSub>
          <DropdownMenuSubTrigger className="gap-2">
            <SunMoonIcon size={16} />
            Theme
          </DropdownMenuSubTrigger>
          <DropdownMenuPortal>
            <DropdownMenuSubContent>
              <DropdownMenuRadioGroup value={theme} onValueChange={(value) => setTheme(value as Theme)}>
                <DropdownMenuRadioItem value={Theme.Light}>Light</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value={Theme.Dark}>Dark</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value={Theme.System}>System</DropdownMenuRadioItem>
              </DropdownMenuRadioGroup>
            </DropdownMenuSubContent>
          </DropdownMenuPortal>
        </DropdownMenuSub>
        <DropdownMenuSeparator />
        <DropdownMenuItem onClick={() => signOut()}>
          <LogOut />
          Log out
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  )
}

type UserAvatarProps = {
  hasArrow?: boolean
}

export const UserAvatar: React.FC<UserAvatarProps> = ({ hasArrow }) => {
  const { user } = useUser()

  return (
    <>
      <Avatar className="h-8 w-8 rounded-lg">
        <AvatarImage src={user?.imageUrl} alt={user?.fullName || ''} />
        <AvatarFallback className="rounded-lg">{user?.fullName?.split('')?.[0]}</AvatarFallback>
      </Avatar>
      <div className="grid flex-1 text-left text-sm leading-tight">
        <span className="truncate font-semibold">{user?.fullName}</span>
        <span className="truncate text-xs">{user?.primaryEmailAddress?.emailAddress}</span>
      </div>
      {hasArrow && <ChevronsUpDown className="ml-auto size-4" />}
    </>
  )
}
