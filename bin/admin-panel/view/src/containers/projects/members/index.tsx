import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog'
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form'
import { Input } from '@/components/ui/input'
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import { Layout } from '@/layouts'
import { useUser } from '@clerk/clerk-react'
import { zodResolver } from '@hookform/resolvers/zod'
import { DialogClose } from '@radix-ui/react-dialog'
import { map } from 'lodash'
import { MoreHorizontal, TrashIcon } from 'lucide-react'
import { useForm } from 'react-hook-form'
import { z } from 'zod'

enum Role {
  Admin = 'admin',
}

const formSchema = z.object({
  email: z.string().email({
    message: 'Please enter a valid email address.',
  }),
  role: z.string({
    message: 'This field is required.',
  }),
})

const roles = [{ value: Role.Admin, label: 'Admin' }]

export const ProjectsMembers = () => {
  const { user } = useUser()
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: '',
      role: Role.Admin,
    },
  })

  const onSubmit = (values: z.infer<typeof formSchema>) => {
    console.log('--------------------------------------------------------')
    console.log('values', values)
    console.log('--------------------------------------------------------')
  }

  return (
    <Layout>
      <div className="max-w-3xl">
        <div className="grid gap-4">
          <div>
            <p className="mb-1 text-xl font-semibold">Members</p>
            <p className="text-sm text-muted-foreground">
              Manage the members of your project. You can add, remove, and change their roles.
            </p>
          </div>
          <Card className="shadow-sm">
            <CardContent className="pt-6">
              <div className="flex items-center justify-between">
                <div className="flex w-full items-center gap-2 text-sm">
                  <Avatar className="h-8 w-8 rounded-lg">
                    <AvatarImage src={user?.imageUrl} alt={user?.fullName || ''} />
                    <AvatarFallback className="rounded-lg">{user?.fullName?.split('')?.[0]}</AvatarFallback>
                  </Avatar>
                  <div className="grid flex-1 text-sm leading-tight">
                    <span className="truncate font-semibold">{user?.fullName}(You)</span>
                    <span className="truncate text-xs">{user?.primaryEmailAddress?.emailAddress}</span>
                  </div>
                  <div className="flex items-center gap-6">
                    <p className="text-xs text-muted-foreground">Owner</p>
                    <DropdownMenu>
                      <DropdownMenuTrigger asChild>
                        <Button variant="ghost" size="icon">
                          <MoreHorizontal />
                        </Button>
                      </DropdownMenuTrigger>
                      <DropdownMenuContent className="w-48" side="bottom" align="end">
                        <DropdownMenuItem className="text-red-500">
                          <TrashIcon />
                          <span>Remove</span>
                        </DropdownMenuItem>
                      </DropdownMenuContent>
                    </DropdownMenu>
                  </div>
                </div>
              </div>
            </CardContent>
            <CardFooter>
              <Dialog>
                <DialogTrigger>
                  <Button>Invite Member</Button>
                </DialogTrigger>
                <DialogContent>
                  <DialogHeader>
                    <DialogTitle>Invite Team Members</DialogTitle>
                    <DialogDescription>
                      Invite team members to your project by entering their email address below.
                    </DialogDescription>
                  </DialogHeader>
                  <Form {...form}>
                    <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4">
                      <FormField
                        control={form.control}
                        name="email"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Email Address</FormLabel>
                            <FormControl>
                              <Input placeholder="example@domain.com" {...field} />
                            </FormControl>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <FormField
                        control={form.control}
                        name="role"
                        render={({ field }) => (
                          <FormItem>
                            <FormLabel>Role</FormLabel>
                            <Select onValueChange={field.onChange} defaultValue={field.value}>
                              <FormControl>
                                <SelectTrigger>
                                  <SelectValue placeholder="Select a role" />
                                </SelectTrigger>
                              </FormControl>
                              <SelectContent>
                                {map(roles, (role) => (
                                  <SelectItem key={role.value} value={role.value}>
                                    {role.label}
                                  </SelectItem>
                                ))}
                              </SelectContent>
                            </Select>
                            <FormMessage />
                          </FormItem>
                        )}
                      />
                      <DialogFooter>
                        <DialogClose asChild>
                          <Button type="button" variant="secondary">
                            Cancel
                          </Button>
                        </DialogClose>
                        <Button type="submit">Send Invitation</Button>
                      </DialogFooter>
                    </form>
                  </Form>
                </DialogContent>
              </Dialog>
            </CardFooter>
          </Card>
        </div>
      </div>
    </Layout>
  )
}
